mod client_message;
mod player_profile;

use crate::{network_client, ANSWER_NAME};
use client_message::{
    query_info_types_enum, DeserializeError, FactionId, LobbyClientMessage, QueryClientStatus,
};

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;

#[repr(u8)]
#[rustfmt::skip]
#[expect(non_camel_case_types)]
#[expect(dead_code)]
enum lobby_server_message_types_enum {
    connection_successful             = 0x30, // 1
    invalid_session_id                = 0x31,
    invalid_password                  = 0x32,
    connect_to_match_server           = 0x33, // '3' ??
    operation_permitted               = 0x34, // '4' ??
    operation_denied                  = 0x35, // '5' ??
    client_status                     = 0x36, // '6' ??
    ping_server_answer                = 0x37, // '7' ??
    lobby_server_invalid_message_type = 0x3F,
}

#[repr(u8)]
#[rustfmt::skip]
#[expect(non_camel_case_types)]
#[expect(dead_code)]
enum lobby_client_message_types_enum {
    set_status_ready_for_match        = 0x20,
    query_client_status               = 0x21,
    inventory_action                  = 0x23,
    shop_action                       = 0x24,
    skills_tree_action                = 0x25,
    lobby_client_sign_in_info         = 0x26, // 1
    discard_playing_order             = 0x27,
    ping_server                       = 0x28,
    lobby_client_invalid_message_type = 0x2F,
}

pub fn handle(stream: TcpStream, buffer: &[u8], bytes_read: usize) {
    let buffer = &mut buffer[0..bytes_read].as_ref();
    match LobbyClientMessage::deserialize(buffer) {
        Ok(LobbyClientMessage::SignInInfo { session_id }) => {
            assert!(buffer.is_empty());

            println!("Received **lobby_client_sign_in_info: {session_id}**");
            let (tx, rx) = mpsc::channel::<QueryClientStatus>();

            std::thread::spawn({
                let stream = stream.try_clone().unwrap();
                move || {
                    run_writer(stream, rx);
                }
            });
            run_reader(stream, tx);
        }
        Ok(msg) => panic!("Received incorrect message. Expected 'SignInInfo': {msg:?}"),
        // @NOTE: If we got an unknown message type, try handling the request in a messaging server
        // Since server sends requests to the same address for both servers
        Err(DeserializeError::UnknownMessageType(_)) => return,
        Err(error) => panic!("{error:?}"),
    }
}

// buffer.push(50); // len
// buffer.push(lobby_server_message_types_enum::client_status as u8);
// buffer.extend([b'A'; 49]);

// CONNECT TO MATCH SERVER
// buffer.push(1 + 1 + lobby_server::ADDRESS.len() as u8 + 4 + 4);
// buffer.push(lobby_server_message_types_enum::connect_to_match_server as u8);
// buffer.push(lobby_server::ADDRESS.len() as u8);
// buffer.extend(lobby_server::ADDRESS.as_bytes());
// buffer.extend(1_u32.to_le_bytes()); // match_id
// buffer.extend(0_u32.to_le_bytes()); // team_id : survarium::game_team_id
fn run_writer(mut stream: TcpStream, rx: mpsc::Receiver<QueryClientStatus>) -> ! {
    stream
        .write(&[
            1_u8, // tcp_msg_len (for response)
            lobby_server_message_types_enum::connection_successful as u8,
        ])
        .unwrap();

    loop {
        let msg = rx.recv().unwrap();
        println!("[writer] Received {msg:?}");
        match msg {
            QueryClientStatus::ClientState => {
                // CLIENT STATE
                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_client_state as u8);
                buffer.push(0); // m_status
                buffer.push("last_status_message".len() as u8);
                buffer.extend(b"last_status_message");

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **client_state**");
            }
            QueryClientStatus::AccountMoney => {
                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_account_money as u8);
                buffer.extend(1_000_000_u32.to_le_bytes()); // generic_money
                buffer.extend(100_000_u32.to_le_bytes()); // premium_money
                buffer.push(70); // skill_points
                buffer.push(ANSWER_NAME.len() as u8);
                buffer.extend(ANSWER_NAME.as_bytes());

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **account_money**");
            }
            QueryClientStatus::PriceItems(faction) => {
                // @NOTE: Seems like no 'generic' items
                let dict_ids: &[u16] = match faction {
                    FactionId::Loners => &[
                        7, 9, 12, 13, 14, 15, 16, 17, 18, 19, 20, 34, 35, 36, 37, 38, 39, 40, 41,
                        42, 43, 44, 45, 46, 47,
                    ],
                    FactionId::Bandits => &[
                        22, 24, 25, 27, 28, 29, 31, 32, 33, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
                    ],
                    // @NOTE: in 001b are not supported
                    FactionId::Army => &[],
                    FactionId::Forest => &[],
                };
                let idx_start = 0;
                let mut buffer = vec![];
                let item_len = dict_ids.len() as u8;

                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_price_items as u8);
                buffer.push(faction as u8); // faction_id
                buffer.extend((item_len as u16).to_le_bytes()); // item_len

                for i in idx_start..idx_start + item_len {
                    let dict_id = dict_ids[i as usize];
                    buffer.extend((dict_id as u16).to_le_bytes()); // 1: item_dict_id
                    buffer.extend((dict_id as u16).to_le_bytes()); // 1: cost
                    buffer.extend(0_u8.to_le_bytes()); // 1: reputation_level
                    buffer.extend(0_u8.to_le_bytes()); // 1: padding
                }

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **price_items**");
            }
            QueryClientStatus::ServicePrices => {
                #[repr(C)]
                struct service_prices_ {
                    reroll_cost: u32,
                    add_profile_cost: u32,
                    rename_account_cost: u32,
                }

                impl service_prices_ {
                    pub fn serialize(&self) -> &[u8] {
                        let ptr = self as *const _ as *const u8;
                        let len = std::mem::size_of::<Self>();
                        unsafe { std::slice::from_raw_parts(ptr, len) }
                    }
                }

                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_service_prices as u8);
                buffer.extend(
                    service_prices_ {
                        reroll_cost: 100,
                        add_profile_cost: 200,
                        rename_account_cost: 300,
                    }
                    .serialize(),
                );

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **service_prices**");
            }

            // @TODO
            QueryClientStatus::EnumerateInventory => {
                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_enumerate_inventory as u8);

                buffer.extend(2_u32.to_le_bytes()); // num of inventory items

                buffer.extend(
                    player_profile::inventory_item_instance {
                        condition_or_stack: 0,
                        amount_in_inventory: 1,
                        id: 1, // ???
                        dict_id: 55,
                    }
                    .serialize(),
                );
                buffer.extend(
                    player_profile::inventory_item_instance {
                        condition_or_stack: 1,
                        amount_in_inventory: 10,
                        id: 2, // ???
                        dict_id: 57,
                    }
                    .serialize(),
                );

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **enumerate_inventory**");
            }

            QueryClientStatus::PlayerReputations => {
                #[repr(C)]
                struct survarium_player_reputation {
                    faction_id: u8,
                    reputation_points: u16,
                }

                impl survarium_player_reputation {
                    pub fn serialize(&self) -> &[u8] {
                        let ptr = self as *const _ as *const u8;
                        let len = std::mem::size_of::<Self>();
                        unsafe { std::slice::from_raw_parts(ptr, len) }
                    }
                }

                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_player_reputations as u8);

                buffer.push(4_u8); // num of reputations
                for (faction_id, reputation_points) in [(1, 100), (2, 200), (3, 300), (4, 400)] {
                    buffer.extend(
                        survarium_player_reputation {
                            faction_id,
                            reputation_points,
                        }
                        .serialize(),
                    );
                }

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **player_reputations**");
            }

            // @TODO
            QueryClientStatus::EnumerateProfiles => {
                let mut buffer = vec![];

                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_enumerate_profiles as u8);

                buffer.push(1); // profiles_counts

                for (i, name) in [
                    "server_profile_1", //"server_profile_2"
                ]
                .iter()
                .enumerate()
                {
                    let i = (i + 1) as u32;
                    buffer.extend((1000 * i).to_le_bytes()); // profile_id
                    buffer.push(name.len() as u8); // profile_name_len
                    buffer.extend(name.as_bytes()); // profile_name_bytes
                }

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **enumerate_profiles**");
            }

            // @TODO
            // @NOTE: Possibly to connect ammo and weapons?
            // Yes!
            QueryClientStatus::ItemsCompatibility => {
                #[repr(C)]
                struct items_compatibility {
                    first_item_dict_id: u16,
                    second_item_dict_id: u16,
                }

                impl items_compatibility {
                    pub fn serialize(&self) -> &[u8] {
                        let ptr = self as *const _ as *const u8;
                        let len = std::mem::size_of::<Self>();
                        unsafe { std::slice::from_raw_parts(ptr, len) }
                    }
                }

                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_items_compatibility as u8);

                buffer.extend(1_u32.to_le_bytes()); // num of compatibilities
                for (first_item_dict_id, second_item_dict_id) in [(7, 9)] {
                    buffer.extend(
                        items_compatibility {
                            first_item_dict_id,
                            second_item_dict_id,
                        }
                        .serialize(),
                    );
                }

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **items_compatibility**");
            }

            // @TODO
            // @NOTE: In which slot what type of weapon can be placed.
            QueryClientStatus::ProfileSlotsRestrictions => {
                #[repr(C)]
                struct profile_slot_restriction {
                    slot_dict_id: u8,
                    category_dict_id: u8,
                }

                impl profile_slot_restriction {
                    pub fn serialize(&self) -> &[u8] {
                        let ptr = self as *const _ as *const u8;
                        let len = std::mem::size_of::<Self>();
                        unsafe { std::slice::from_raw_parts(ptr, len) }
                    }
                }

                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_profile_slots_restrictions as u8);

                buffer.extend(1_u32.to_le_bytes()); // num of compatibilities
                for (slot_dict_id, category_dict_id) in [(1, 2)] {
                    buffer.extend(
                        profile_slot_restriction {
                            slot_dict_id,
                            category_dict_id,
                        }
                        .serialize(),
                    );
                }

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **profile_slot_restrictions**");
            }

            QueryClientStatus::PlayerSkills => {
                let mut buffer = vec![];

                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_player_skills as u8);

                buffer.extend(1800_u32.to_le_bytes()); // total_experience
                buffer.extend(3750_u32.to_le_bytes()); // next_level_experience
                buffer.extend(1000_u32.to_le_bytes()); // prev_level_experience

                buffer.push(5); // player_skills_count
                for (skill_id, skill_points) in [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)] {
                    #[repr(C)]
                    struct player_skill {
                        skill_id: u8,
                        skill_points: u8,
                    }

                    impl player_skill {
                        pub fn serialize(&self) -> &[u8] {
                            let ptr = self as *const _ as *const u8;
                            let len = std::mem::size_of::<Self>();
                            unsafe { std::slice::from_raw_parts(ptr, len) }
                        }
                    }

                    buffer.extend(
                        player_skill {
                            skill_id,
                            skill_points,
                        }
                        .serialize(),
                    );
                }

                // `fill_skills_tree` function crashes when `empty` perk is passed
                // buffer.push(1); // player_perks_count
                // buffer.push(0); // empty

                buffer.push(0); // player_perks_count

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **player_skills**");
            }

            // This should return binary config for player skill tree.
            // Hopefully this is not used anywhere, so we will return an empty config
            // this->m_skills_tree_config,
            QueryClientStatus::PlayerSkillsTree => {
                let mut buffer = vec![];

                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(query_info_types_enum::q_player_skills_tree as u8);

                const SKILLS_TREE: &[u8] = include_bytes!("../../../../resources/skills_tree.bin");
                buffer.extend(SKILLS_TREE);

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **player_skills_tree**");
            }
            // {

            //     buffer.extend(0x18_u64.to_le_bytes()); // data
            //     buffer.extend(0_u64.to_le_bytes()); // id
            //     buffer.extend(0_u32.to_le_bytes()); // id_crc
            //     buffer.extend(4_u16.to_le_bytes()); // binary_type: table_named
            //     buffer.extend(0_u16.to_le_bytes()); // count

            // }

            // [writer] Received PlayerSkills
            // [writer] Received ProfileSlotsRestrictions

            // item_category:
            // 1 - helmet
            // 2 - respirator
            // 3 - torso
            // 4 - backpack
            // 5 - legs
            // 6 - gloves
            // 7 - boots
            // 9 - ammo    | ammo_5.45x39_fmj
            // 10 - meds   | painkiller, bandages, medkit
            // 11 - traps
            // 12 - arts   | both here
            // 13 - weapon | rem_700, toz_122
            // 14 - weapon | rem_870, toz_34, toz_66
            // 15 - weapon | uzi, ak_74u, vituaz
            // 17 - weapon | tt_33, fort_17, magnum
            // 18 - ammo   | ammo_7.62x51_ap, ammo_7.62x51
            // 19 - ammo   | ammo_12mm_slug, ammo_12mm_buck, ammo_12mm_buck2
            // 20 - ammo   | ammo_9x19p_fmj, ammo_9x19p_hp, ammo_7.62x25, ammo_9x18_makarov, ammo_.357m
            // 22 - scope
            //
            //
            // restriction_0[2]
            //   category_id: 1
            //   profile_slot_id: 0
            // restriction_1[2]
            //   category_id: 2
            //   profile_slot_id: 1
            // restriction_2[2]
            //   category_id: 3
            //   profile_slot_id: 2
            // restriction_3[2]
            //   category_id: 4
            //   profile_slot_id: 3
            // restriction_4[2]
            //   category_id: 5
            //   profile_slot_id: 4
            // restriction_5[2]
            //   category_id: 6
            //   profile_slot_id: 5
            // restriction_6[2]
            //   category_id: 7
            //   profile_slot_id: 6

            // restriction_23[2]
            //   category_id: 8 ???
            //   profile_slot_id: 7
            //   profile_slot_id: 10

            // restriction_57[2]
            //   category_id: 9
            //   profile_slot_id: 8
            //   profile_slot_id: 9
            //   profile_slot_id: 11
            //   profile_slot_id: 12
            //   profile_slot_id: 19
            //   profile_slot_id: 20
            //   profile_slot_id: 21
            //   profile_slot_id: 22

            // restriction_45[2]
            //   category_id: 10
            //   profile_slot_id: 13
            //   profile_slot_id: 14
            //   profile_slot_id: 15
            //   profile_slot_id: 16
            //   profile_slot_id: 17
            //   profile_slot_id: 18

            // restriction_46[2]
            //   category_id: 11
            //   profile_slot_id: 13
            //   profile_slot_id: 14
            //   profile_slot_id: 15
            //   profile_slot_id: 16
            //   profile_slot_id: 17
            //   profile_slot_id: 18

            // restriction_8[2]
            //   category_id: 13
            //   profile_slot_id: 7
            //   profile_slot_id: 10

            // restriction_9[2]
            //   category_id: 14
            //   profile_slot_id: 7
            //   profile_slot_id: 10

            // restriction_26[2]
            //   category_id: 15
            //   profile_slot_id: 7
            //   profile_slot_id: 10

            // restriction_11[2]
            //   category_id: 16
            //   profile_slot_id: 7
            //   profile_slot_id: 10

            // restriction_12[2]
            //   category_id: 17
            //   profile_slot_id: 7
            //   profile_slot_id: 10

            // restriction_36[2]
            //   category_id: 18
            //   profile_slot_id: 8
            //   profile_slot_id: 9
            //   profile_slot_id: 11
            //   profile_slot_id: 12
            //   profile_slot_id: 19
            //   profile_slot_id: 20
            //   profile_slot_id: 21
            //   profile_slot_id: 22

            // restriction_36[2]
            //   category_id: 19
            //   profile_slot_id: 8
            //   profile_slot_id: 9
            //   profile_slot_id: 11
            //   profile_slot_id: 12
            //   profile_slot_id: 19
            //   profile_slot_id: 20
            //   profile_slot_id: 21
            //   profile_slot_id: 22

            // restriction_16[2]
            //   category_id: 20
            //   profile_slot_id: 8
            //   profile_slot_id: 9
            //   profile_slot_id: 11
            //   profile_slot_id: 12
            //   profile_slot_id: 19
            //   profile_slot_id: 20
            //   profile_slot_id: 21
            //   profile_slot_id: 22

            // restriction_17[2]
            //   category_id: 21
            //   profile_slot_id: 8
            //   profile_slot_id: 9
            //   profile_slot_id: 11
            //   profile_slot_id: 12
            //   profile_slot_id: 19
            //   profile_slot_id: 20
            //   profile_slot_id: 21
            //   profile_slot_id: 22

            //
            QueryClientStatus::ProfileContents { profile_id } => {
                dbg!(profile_id);
            }
        }
    }
}

fn run_reader(mut stream: TcpStream, tx: mpsc::Sender<QueryClientStatus>) -> ! {
    let mut buffer = [0_u8; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer[0..]).unwrap();

        let msgs_buffer = &mut buffer[0..bytes_read].as_ref();

        while !msgs_buffer.is_empty() {
            let client_message = LobbyClientMessage::deserialize(msgs_buffer);
            match client_message {
                Ok(LobbyClientMessage::PingServer { current_time: _ }) => (),
                Ok(LobbyClientMessage::QueryClientStatus(status)) => {
                    println!("[reader] Received client status request {status:?}");
                    tx.send(status).unwrap()
                }
                Ok(LobbyClientMessage::SignInInfo { session_id: _ }) => (),
                Err(error) => {
                    println!("{error:?}");
                    println!("{msgs_buffer:?}");
                }
            }
        }
    }
}
