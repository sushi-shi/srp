use num_traits::FromPrimitive;

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive)]
#[expect(non_camel_case_types)]
#[repr(u8)]
enum lobby_client_message_types_enum {
    set_status_ready_for_match        = 0x20,
    query_client_status               = 0x21, // 33
    inventory_action                  = 0x23,
    shop_action                       = 0x24, // 36
    skills_tree_action                = 0x25, // 37
    lobby_client_sign_in_info         = 0x26,
    discard_playing_order             = 0x27,
    ping_server                       = 0x28,
    lobby_client_invalid_message_type = 0x2F,
}

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive, Debug, PartialEq)]
#[expect(non_camel_case_types)]
#[repr(u8)]
pub enum query_info_types_enum {
    q_client_state               = 0x0, //
    q_enumerate_profiles         = 0x1,
    q_profile_contents           = 0x2,
    q_enumerate_inventory        = 0x3,
    q_profile_slots_restrictions = 0x4, // ? (check debugger)
    q_items_compatibility        = 0x5, // ?
    q_price_items                = 0x6, // +
    q_account_money              = 0x7,
    q_player_skills              = 0x8,
    q_player_skills_tree         = 0x9, // ?
    q_service_prices             = 0xA, // +
    q_player_reputations         = 0xB,
}

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive, Debug, PartialEq)]
#[expect(non_camel_case_types)]
#[repr(u8)]
pub enum inventory_events_enum {
    item_moved_to_slot = 0x0,
}

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive, Debug, PartialEq)]
#[expect(non_camel_case_types)]
#[repr(u8)]
pub enum shop_events_enum {
    item_bought = 0x0,
}

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive, Debug, PartialEq)]
#[expect(non_camel_case_types)]
#[repr(u8)]
pub enum skills_tree_events_enum {
    player_skills_changed = 0x0,
    reroll_skills         = 0x1,
    player_perks_changed  = 0x2,
}

#[derive(Debug, PartialEq)]
pub enum LobbyClientMessage {
    ReadyForMatch { profile_id: u32 },
    QueryClientStatus(QueryClientStatus),
    InventoryAction,
    ShopAction(ShopAction),

    // 5 bytes
    // login_client_message_types_enum::lobby_client_sign_in_info
    SignInInfo { session_id: u32 },
    // 5 bytes
    // login_client_message_types_enum::ping_server
    PingServer { alive_seconds: u32 },
}

#[derive(Debug, PartialEq)]
pub enum QueryClientStatus {
    ClientState,
    EnumerateProfiles,
    ProfileContents { profile_id: u32 },
    EnumerateInventory,
    ProfileSlotsRestrictions,
    ItemsCompatibility,
    PriceItems(FactionId),
    AccountMoney,
    PlayerSkills,
    PlayerSkillsTree,
    ServicePrices,
    PlayerReputations,
}

#[derive(Debug, PartialEq)]
pub enum InventoryAction {
    Empty,
    Equip,
}

#[derive(Debug, PartialEq)]
pub enum ShopAction {
    Buy {
        dict_id: u16,
        amount: u16,
        _idk: u16,
        faction_id: FactionId,
    },
}

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive, Debug, PartialEq)]
#[repr(u8)]
pub enum FactionId {
    Loners  = 0x1,
    Bandits = 0x2,
    Army    = 0x3,
    Forest  = 0x4,
}
// Was used in `on_lobby_packed_received`
// pub enum FactionId {
//     Loners  = 0b0001,
//     Bandits = 0b0010,
//     Army    = 0b0100,
//     Forest  = 0b1000,
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DeserializeError {
    NotEnoughInput,
    UnknownMessageType(u8),
    Todo,
    IncorrectInput,
}

impl LobbyClientMessage {
    pub fn deserialize(out_buffer: &mut &[u8]) -> Result<Self, DeserializeError> {
        if out_buffer.is_empty() {
            return Err(DeserializeError::NotEnoughInput);
        }

        let tcp_msg_len = out_buffer[0] as usize;
        // tcp_msg_len doesn't include itself
        if out_buffer.len() < tcp_msg_len + 1 {
            return Err(DeserializeError::NotEnoughInput);
        }

        let msg_type = out_buffer[1];
        let Some(msg_type) = lobby_client_message_types_enum::from_u8(msg_type) else {
            return Err(DeserializeError::UnknownMessageType(msg_type));
        };

        // [len | msg_type | ...... ]
        // 0    1          2       1 + len
        //      |------- len -------|
        //                 | buffer |
        let buffer = &out_buffer[2..tcp_msg_len + 1];

        let result = match msg_type {
            lobby_client_message_types_enum::set_status_ready_for_match => {
                if buffer.len() != 4 {
                    return Err(DeserializeError::IncorrectInput);
                }
                let profile_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                Ok(LobbyClientMessage::ReadyForMatch { profile_id })
            }

            // [len | msg_type | query_type | ... ]
            // 0    1          2            3     1 + len
            lobby_client_message_types_enum::query_client_status => {
                let query_info_type = buffer[0];
                let query_info_type = query_info_types_enum::from_u8(query_info_type)
                    .ok_or(DeserializeError::IncorrectInput)?;

                let buffer = &buffer[1..];

                let query_client_status = match query_info_type {
                    query_info_types_enum::q_client_state => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::ClientState
                    }
                    query_info_types_enum::q_enumerate_profiles => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::EnumerateProfiles
                    }
                    query_info_types_enum::q_profile_contents => {
                        if buffer.len() != 4 {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        let profile_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                        QueryClientStatus::ProfileContents { profile_id }
                    }
                    query_info_types_enum::q_enumerate_inventory => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::EnumerateInventory
                    }
                    query_info_types_enum::q_profile_slots_restrictions => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::ProfileSlotsRestrictions
                    }
                    query_info_types_enum::q_items_compatibility => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::ItemsCompatibility
                    }
                    query_info_types_enum::q_price_items => {
                        if buffer.len() != 1 {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        let faction_id = buffer[0];
                        let faction_id = FactionId::from_u8(faction_id)
                            .ok_or(DeserializeError::IncorrectInput)?;
                        QueryClientStatus::PriceItems(faction_id)
                    }
                    query_info_types_enum::q_account_money => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::AccountMoney
                    }
                    query_info_types_enum::q_player_skills => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::PlayerSkills
                    }
                    query_info_types_enum::q_player_skills_tree => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::PlayerSkillsTree
                    }
                    query_info_types_enum::q_service_prices => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::ServicePrices
                    }
                    query_info_types_enum::q_player_reputations => {
                        if buffer != &[0, 0, 0] {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        QueryClientStatus::PlayerReputations
                    }
                };

                Ok(Self::QueryClientStatus(query_client_status))
            }

            // FFFFFFFF enum lobby::inventory_events_enum : __int32
            // FFFFFFFF {
            // FFFFFFFF     item_moved_to_slot = 0x0,
            // FFFFFFFF };

            // uzi equip 10 (or 7)
            // [25, 35, 0, 1, 64, 13, 3, 0, 16, 39, 0, 0, 55, 0, 0, 0, 100, 0, 0, 0, 7, 0, 0, 0, 1, 0]
            // [25, 35, 0, 1, 64, 13, 3, 0, 10, 0, 0, 0, 55, 0, 0, 0, 100, 0, 0, 0, 7, 0, 0, 0, 1, 0]
            //
            // [25, 35, 0, 1, 64, 13, 3, 0, 16, 39, 0, 0, 55, 0, 0, 0, 100, 0, 0, 0, 10, 0, 0, 0, 1, 0]
            //
            // unequip
            // [25, 35, 0, 1, 128, 26, 6, 0, 7, 0, 0, 0, 55, 0, 0, 0, 7, 0, 0, 0, 100, 0, 0, 0, 1, 0]
            lobby_client_message_types_enum::inventory_action => {
                if buffer != &[0, 0] {
                    return Err(DeserializeError::IncorrectInput);
                }
                Ok(Self::InventoryAction)
            }

            lobby_client_message_types_enum::shop_action => {
                let Ok(buffer) = buffer.try_into() else {
                    return Err(DeserializeError::IncorrectInput);
                };
                let buffer: &[u8; 9] = buffer;
                let (action_type, dict_id, amount, _idk, faction_id) =
                    arrayref::array_refs![buffer, 1, 2, 2, 2, 2];

                let action_type = shop_events_enum::from_u8(action_type[0])
                    .ok_or(DeserializeError::IncorrectInput)?;
                match action_type {
                    shop_events_enum::item_bought => {
                        let dict_id = u16::from_le_bytes(*dict_id);
                        let amount = u16::from_le_bytes(*amount);
                        let _idk = u16::from_le_bytes(*_idk);
                        let faction_id = u16::from_le_bytes(*faction_id);

                        let faction_id = FactionId::from_u16(faction_id)
                            .ok_or(DeserializeError::IncorrectInput)?;

                        Ok(Self::ShopAction(ShopAction::Buy {
                            dict_id,
                            amount,
                            _idk,
                            faction_id,
                        }))
                    }
                }
            }
            lobby_client_message_types_enum::skills_tree_action => Err(DeserializeError::Todo),
            lobby_client_message_types_enum::lobby_client_sign_in_info => match buffer.len() {
                4 => {
                    let session_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                    Ok(Self::SignInInfo { session_id })
                }
                _ => Err(DeserializeError::NotEnoughInput),
            },
            lobby_client_message_types_enum::discard_playing_order => Err(DeserializeError::Todo),
            lobby_client_message_types_enum::ping_server => match buffer.len() {
                4 => {
                    let alive_seconds = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                    Ok(Self::PingServer { alive_seconds })
                }
                _ => Err(DeserializeError::NotEnoughInput),
            },
            lobby_client_message_types_enum::lobby_client_invalid_message_type => {
                Err(DeserializeError::Todo)
            }
        };
        if result.is_ok() {
            *out_buffer = &out_buffer[tcp_msg_len + 1..];
        }
        result
    }
}

#[test]
fn parses_single_query_client_msg() {
    let buffer: &[u8] = &[5, 33, 10, 0, 0, 0];
    let defacto = LobbyClientMessage::deserialize(&mut buffer.as_ref()).unwrap();
    let dejure = LobbyClientMessage::QueryClientStatus(QueryClientStatus::ServicePrices);
    assert_eq!(defacto, dejure);
}

#[test]
fn parses_multiple_query_client_msg() {
    #[rustfmt::skip]
    let buffer: &[u8] = &[
        5, 33, 5, 0, 0, 0,
        5, 33, 9, 0, 0, 0,
        5, 33, 10, 0, 0, 0,
        3, 33, 6, 1,
        3, 33, 6, 2,
        3, 33, 6, 3,
        3, 33, 6, 4,
        5, 33, 0, 0, 0, 0,
    ];
    let buffer = &mut buffer.as_ref();

    let msg_num_dejure = 8;
    let mut msg_num_defacto = 0;
    let mut msgs = vec![];

    while !buffer.is_empty() {
        msg_num_defacto += 1;
        msgs.push(LobbyClientMessage::deserialize(buffer).unwrap());
    }

    assert_eq!(msg_num_defacto, msg_num_dejure);

    assert_eq!(
        msgs[2],
        LobbyClientMessage::QueryClientStatus(QueryClientStatus::ServicePrices),
    );

    assert_eq!(
        msgs[4],
        LobbyClientMessage::QueryClientStatus(QueryClientStatus::PriceItems(FactionId::Bandits)),
    );

    assert_eq!(
        msgs[7],
        LobbyClientMessage::QueryClientStatus(QueryClientStatus::ClientState)
    );
}

#[test]
fn parses_inventory_actions() {
    // [25, 35, 0, 1, 64, 13, 3, 0, 7, 0, 0, 0, 55, 0, 0, 0, 100, 0, 0, 0, 10, 0, 0, 0, 1, 0]
    //
    let buffer: &[u8] = &[5, 33, 10, 0, 0, 0];
    let defacto = LobbyClientMessage::deserialize(&mut buffer.as_ref()).unwrap();
    let dejure = LobbyClientMessage::QueryClientStatus(QueryClientStatus::ServicePrices);
    assert_eq!(defacto, dejure);
}

#[test]
fn parses_set_ready_for_match() {
    let buffer: &[u8] = &[5, 32, 1, 0, 0, 0];
    let defacto = LobbyClientMessage::deserialize(&mut buffer.as_ref()).unwrap();
    let dejure = LobbyClientMessage::ReadyForMatch { profile_id: 1 };
    assert_eq!(defacto, dejure);
}

#[test]
fn parses_shop_actions() {
    // Bying item
    let buffer: &[u8] = &[10, 36, 0, 7, 0, 220, 5, 0, 0, 1, 0];
    let defacto = LobbyClientMessage::deserialize(&mut buffer.as_ref()).unwrap();
    let dejure = LobbyClientMessage::ShopAction(ShopAction::Buy {
        dict_id: 7,
        amount: 1500,
        _idk: 0,
        faction_id: FactionId::Loners,
    });
    assert_eq!(defacto, dejure);
}

// @TODO
// #[test]
// fn parses_skills_tree_actions() {
//     // Setting skill
//     // [14, 37, 0, 5, 1, 0, 2, 0, 3, 0, 4, 0, 5, 3, 0]
//     // [17, 37, 0, 5, 1, 0, 2, 20, 3, 10, 4, 20, 5, 20, 3, 14, 28, 35]
//     // Resetting skill tree
//     // [2, 37, 1]
//     let buffer: &[u8] = &[5, 33, 10, 0, 0, 0];
//     let defacto = LobbyClientMessage::deserialize(&mut buffer.as_ref()).unwrap();
//     let dejure = LobbyClientMessage::QueryClientStatus(QueryClientStatus::ServicePrices);
//     assert_eq!(defacto, dejure);
// }

// struct price_item {
//     item_dict_id: u16,
//     cost: u16,
//     reputation_level: u8,
// }
