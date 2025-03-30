mod client_message;
mod player_profile;

use crate::network_client::Packet;
use crate::ANSWER_NAME;
use client_message::{
    query_info_types_enum, DeserializeError, FactionId, LobbyClientMessage, QueryClientStatus,
    ShopAction,
};

use std::io::Read;
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

pub fn handle(mut stream: TcpStream, buffer: &[u8], bytes_read: usize) -> TcpStream {
    let buffer = &mut buffer[0..bytes_read].as_ref();
    match LobbyClientMessage::deserialize(buffer) {
        Ok(LobbyClientMessage::SignInInfo { session_id }) => {
            println!("Received **lobby_client_sign_in_info: {session_id}**");

            assert!(buffer.is_empty());
            let mut packet = Packet::new();
            packet.push(lobby_server_message_types_enum::connection_successful as u8);
            packet.send(&mut stream);

            let (tx, rx) = mpsc::channel();
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
        Err(DeserializeError::UnknownMessageType(_)) => return stream,

        Err(error) => panic!("{error:?}"),
    }
}

// packet.push(50); // len
// packet.push(lobby_server_message_types_enum::client_status as u8);
// packet.extend([b'A'; 49]);

// CONNECT TO MATCH SERVER
// packet.push(1 + 1 + lobby_server::ADDRESS.len() as u8 + 4 + 4);
// packet.push(lobby_server_message_types_enum::connect_to_match_server as u8);
// packet.push(lobby_server::ADDRESS.len() as u8);
// packet.extend(lobby_server::ADDRESS.as_bytes());
// packet.extend(1_u32.to_le_bytes()); // match_id
// packet.extend(0_u32.to_le_bytes()); // team_id : survarium::game_team_id
fn run_writer(mut stream: TcpStream, rx: mpsc::Receiver<LobbyClientMessage>) -> ! {
    let stream = &mut stream;

    let mut id = 0_u32;
    loop {
        let msg = rx.recv().unwrap();

        println!("[writer] Received {msg:?}");
        match msg {
            LobbyClientMessage::SignInInfo { session_id: _ } => (),
            LobbyClientMessage::PingServer { alive_seconds: _ } => (),

            LobbyClientMessage::ReadyForMatch { profile_id: _ } => (),

            LobbyClientMessage::InventoryAction => {
                let mut packet = Packet::new();
                packet.push(lobby_server_message_types_enum::operation_permitted as u8);
                packet.push(lobby_client_message_types_enum::inventory_action as u8);

                packet.send(stream);
                println!("[writer] Wrote **permitted inventory action**");
            }

            LobbyClientMessage::ShopAction(ShopAction::Buy {
                dict_id,
                amount,
                _idk,
                faction_id: _,
            }) => {
                let condition_or_stack = amount as u32;

                let mut packet = Packet::new();
                packet.push(lobby_server_message_types_enum::operation_permitted as u8);
                packet.push(lobby_client_message_types_enum::inventory_action as u8);
                packet.push(1 as u8); // shop_events_enum_response

                packet.extend(dict_id.to_le_bytes());
                packet.extend(id.to_le_bytes());
                packet.extend(condition_or_stack.to_le_bytes());

                packet.send(stream);
                println!("[writer] Wrote **permitted shop action**");

                id += 1;
            }

            LobbyClientMessage::QueryClientStatus(status) => {
                match status {
                    QueryClientStatus::ClientState => {
                        let m_status = 0_u8;

                        let mut packet = Packet::new();
                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_client_state as u8);
                        packet.push(m_status);
                        packet.push("last_status_message".len() as u8);
                        packet.extend(b"last_status_message");

                        packet.send(stream);
                        println!("[writer] Wrote **client_state**");
                    }
                    QueryClientStatus::AccountMoney => {
                        let generic_money = 1_000_000_u32;
                        let premium_money = 100_000_u32;
                        let skill_points = 70_u8;

                        let mut packet = Packet::new();
                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_account_money as u8);
                        packet.extend(generic_money.to_le_bytes());
                        packet.extend(premium_money.to_le_bytes());
                        packet.push(skill_points);
                        packet.push(ANSWER_NAME.len() as u8);
                        packet.extend(ANSWER_NAME.as_bytes());

                        packet.send(stream);
                        println!("[writer] Wrote **account_money**");
                    }
                    QueryClientStatus::PriceItems(faction) => {
                        let dict_ids: &[u16] = match faction {
                            FactionId::Loners => &[
                                7, 9, 12, 13, 14, 15, 16, 17, 18, 19, 20, 34, 35, 36, 37, 38, 39,
                                40, 41, 42, 43, 44, 45, 46, 47,
                            ],
                            FactionId::Bandits => &[
                                22, 24, 25, 27, 28, 29, 31, 32, 33, 48, 49, 50, 51, 52, 53, 55, 56,
                                64, 65, 66, 67, 68, 70, 71, 72, 73,
                            ],

                            // Scopes and artefacts: 54, 57, 69

                            // @NOTE: in 001b are not supported
                            FactionId::Army => &[],
                            FactionId::Forest => &[],
                        };
                        let idx_start = 0;
                        let mut packet = Packet::new();
                        let item_len = dict_ids.len() as u8;

                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_price_items as u8);
                        packet.push(faction as u8); // faction_id
                        packet.extend((item_len as u16).to_le_bytes()); // item_len

                        for i in idx_start..idx_start + item_len {
                            let dict_id = dict_ids[i as usize];
                            packet.extend((dict_id as u16).to_le_bytes()); // 1: item_dict_id
                            packet.extend((dict_id as u16).to_le_bytes()); // 1: cost
                            packet.extend(0_u8.to_le_bytes()); // 1: reputation_level
                            packet.extend(0_u8.to_le_bytes()); // 1: padding
                        }

                        packet.send(stream);
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

                        let mut packet = Packet::new();
                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_service_prices as u8);
                        packet.extend(
                            service_prices_ {
                                reroll_cost: 100,
                                add_profile_cost: 200,
                                rename_account_cost: 300,
                            }
                            .serialize(),
                        );

                        packet.send(stream);
                        println!("[writer] Wrote **service_prices**");
                    }

                    // @TODO
                    QueryClientStatus::EnumerateInventory => {
                        let i = |id, dict_id, condition_or_stack| {
                            player_profile::inventory_item_instance {
                                condition_or_stack,
                                amount_in_inventory: 1,
                                id,
                                dict_id,
                            }
                        };
                        let items = [
                            i(1, 24, 10), // boots
                            i(2, 40, 20), // gloves
                            i(3, 46, 30), // legs
                            i(4, 27, 40), // helmet
                            i(5, 43, 50), // resp
                            i(6, 48, 60), // torso
                            i(7, 9, 70),  // back
                            //
                            i(8, 65, 80),   // painkiller
                            i(9, 66, 90),   // bandages
                            i(10, 67, 100), // medkit
                            i(11, 68, 110), // traps
                            //
                            i(12, 55, 120), // uzi
                            i(13, 55, 130), // uzi
                        ];
                        let items_len: u32 = items.len().try_into().unwrap();

                        let mut packet = Packet::new();
                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_enumerate_inventory as u8);

                        packet.extend(items_len.to_le_bytes());

                        for item in items {
                            packet.extend(item.serialize());
                        }

                        packet.send(stream);
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

                        let mut packet = Packet::new();
                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_player_reputations as u8);

                        packet.push(4_u8); // num of reputations
                        for (faction_id, reputation_points) in
                            [(1, 100), (2, 200), (3, 300), (4, 400)]
                        {
                            packet.extend(
                                survarium_player_reputation {
                                    faction_id,
                                    reputation_points,
                                }
                                .serialize(),
                            );
                        }

                        packet.send(stream);
                        println!("[writer] Wrote **player_reputations**");
                    }

                    QueryClientStatus::EnumerateProfiles => {
                        let mut packet = Packet::new();

                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_enumerate_profiles as u8);

                        packet.push(3); // profiles_counts

                        for (i, name) in
                            ["server_profile_1", "server_profile_2", "server_profile_3"]
                                .iter()
                                .enumerate()
                        {
                            let i = 200_000_u32 * ((i + 1) as u32);
                            packet.extend(i.to_le_bytes()); // profile_id
                            packet.push(name.len() as u8); // profile_name_len
                            packet.extend(name.as_bytes()); // profile_name_bytes
                        }

                        packet.send(stream);
                        println!("[writer] Wrote **enumerate_profiles**");
                    }

                    // Used to connect ammo and weapons
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

                        let mut packet = Packet::new();
                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_items_compatibility as u8);

                        let compats: &[(u16, u16)] = &[
                            (12, 51),
                            (12, 52),
                            (13, 7),
                            (14, 51),
                            (14, 52),
                            (15, 22),
                            (15, 72),
                            (16, 22),
                            (16, 72),
                            (17, 22),
                            (17, 72),
                            (18, 50),
                            (19, 53),
                            (19, 71),
                            (55, 53),
                            (55, 71),
                            (56, 20),
                            (64, 70),
                        ];

                        let compats_num: u32 = compats.len().try_into().unwrap();
                        packet.extend(compats_num.to_le_bytes()); // num of compatibilities
                        for (first_item_dict_id, second_item_dict_id) in compats.iter().copied() {
                            packet.extend(
                                items_compatibility {
                                    first_item_dict_id,
                                    second_item_dict_id,
                                }
                                .serialize(),
                            );
                        }

                        packet.send(stream);
                        println!("[writer] Wrote **items_compatibility**");
                    }

                    // In which slot what type of weapon can be placed.
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

                        let mut packet = Packet::new();
                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_profile_slots_restrictions as u8);

                        // (category_id, profile_slot_id)
                        #[rustfmt::skip]
                        let restricts: &[(u8, u8)] = &[
                            (1, 0),
                            (2, 1),
                            (3, 2),
                            (4, 3),
                            (5, 4),
                            (6, 5),
                            (7, 6),
                            //
                            (9, 8),
                            (9, 9),
                            (9, 11),
                            (9, 12),
                            //
                            (10, 13), (10, 14), (10, 15), (10, 16), (10, 17), (10, 18),
                            (11, 13), (11, 14), (11, 15), (11, 16), (11, 17), (11, 18),
                            //
                            (13, 7), (13, 10),
                            (14, 7), (14, 10),
                            (15, 7), (15, 10),
                            (17, 7), (17, 10),
                            //
                            (18, 13), (18, 14), (18, 15), (18, 16), (18, 17), (18, 18),
                            (19, 13), (19, 14), (19, 15), (19, 16), (19, 17), (19, 18),
                            (20, 13), (20, 14), (20, 15), (20, 16), (20, 17), (20, 18),
                        ];

                        let restricts_num: u32 = restricts.len().try_into().unwrap();
                        packet.extend(restricts_num.to_le_bytes()); // num of compatibilities
                        for (category_dict_id, slot_dict_id) in restricts.iter().copied() {
                            packet.extend(
                                profile_slot_restriction {
                                    slot_dict_id,
                                    category_dict_id,
                                }
                                .serialize(),
                            );
                        }

                        packet.send(stream);
                        println!("[writer] Wrote **profile_slot_restrictions**");
                    }

                    QueryClientStatus::PlayerSkills => {
                        let mut packet = Packet::new();

                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_player_skills as u8);

                        packet.extend(1800_u32.to_le_bytes()); // total_experience
                        packet.extend(3750_u32.to_le_bytes()); // next_level_experience
                        packet.extend(1000_u32.to_le_bytes()); // prev_level_experience

                        packet.push(5); // player_skills_count
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

                            packet.extend(
                                player_skill {
                                    skill_id,
                                    skill_points,
                                }
                                .serialize(),
                            );
                        }

                        packet.push(0); // player_perks_count

                        packet.send(stream);
                        println!("[writer] Wrote **player_skills**");
                    }

                    QueryClientStatus::PlayerSkillsTree => {
                        let mut packet = Packet::new();

                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_player_skills_tree as u8);

                        const SKILLS_TREE: &[u8] =
                            include_bytes!("../../../../resources/skills_tree.bin");
                        packet.extend(SKILLS_TREE);

                        packet.send(stream);
                        println!("[writer] Wrote **player_skills_tree**");
                    }

                    QueryClientStatus::ProfileContents { profile_id } => {
                        let mut packet = Packet::new();
                        packet.push(lobby_server_message_types_enum::client_status as u8);
                        packet.push(query_info_types_enum::q_profile_contents as u8);

                        packet.extend(
                            player_profile::player_profile::new(
                                100_000_u32,
                                profile_id,
                                match profile_id {
                                    200_000 => "server_profile_1",
                                    400_000 => "server_profile_2",
                                    600_000 => "server_profile_3",
                                    _ => unreachable!(),
                                },
                            )
                            .deserialize(),
                        );

                        packet.send(stream);
                        println!("[writer] Wrote **profile_contents**");
                    }
                }
            }
        }
    }
}

fn run_reader(mut stream: TcpStream, tx: mpsc::Sender<LobbyClientMessage>) -> ! {
    let mut buffer = [0_u8; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer[0..]).unwrap();

        let msgs_buffer = &mut buffer[0..bytes_read].as_ref();

        while !msgs_buffer.is_empty() {
            let client_message = LobbyClientMessage::deserialize(msgs_buffer);
            match client_message {
                Ok(LobbyClientMessage::SignInInfo { session_id: _ }) => (),
                Ok(LobbyClientMessage::PingServer { alive_seconds: _ }) => (),
                Ok(msg) => tx.send(msg).unwrap(),
                Err(error) => {
                    println!("{error:?}");
                    println!("{msgs_buffer:?}");
                    break;
                }
            }
        }
    }
}
