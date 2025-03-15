use num_traits::FromPrimitive;

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive)]
#[allow(non_camel_case_types)]
#[repr(u8)]
enum lobby_client_message_types_enum {
    set_status_ready_for_match        = 0x20,
    query_client_status               = 0x21, // 3 | 33
    inventory_action                  = 0x23,
    shop_action                       = 0x24,
    skills_tree_action                = 0x25, //   | 37
    lobby_client_sign_in_info         = 0x26, // 1
    discard_playing_order             = 0x27,
    ping_server                       = 0x28, // 2
    lobby_client_invalid_message_type = 0x2F,
}

#[derive(Debug, PartialEq)]
pub enum LobbyClientMessage {
    // 5 bytes
    // login_client_message_types_enum::lobby_client_sign_in_info
    SignInInfo { session_id: u32 },
    // 5 bytes
    // login_client_message_types_enum::ping_server
    PingServer { current_time: u32 },
    //
    QueryClientStatus(QueryClientStatus),
}

#[derive(Debug, PartialEq)]
pub enum QueryClientStatus {
    ClientState,
    EnumerateProfiles,
    ProfileContents,
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

// @TODO: Maybe IDs are incorrect. Order is from the wiki
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

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum QueryInfoTypes {
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

        let buffer = &out_buffer[2..];
        let buffer_len = tcp_msg_len - 1;
        let result = match msg_type {
            lobby_client_message_types_enum::set_status_ready_for_match => {
                Err(DeserializeError::Todo)
            }
            lobby_client_message_types_enum::query_client_status => {
                let query_info_type = buffer[0];
                let query_info_type = QueryInfoTypes::from_u8(query_info_type)
                    .ok_or(DeserializeError::IncorrectInput)?;

                let query_client_status = match query_info_type {
                    QueryInfoTypes::q_client_state => QueryClientStatus::ClientState,
                    QueryInfoTypes::q_enumerate_profiles => QueryClientStatus::EnumerateProfiles,
                    QueryInfoTypes::q_profile_contents => QueryClientStatus::ProfileContents,
                    QueryInfoTypes::q_enumerate_inventory => QueryClientStatus::EnumerateInventory,
                    QueryInfoTypes::q_profile_slots_restrictions => {
                        QueryClientStatus::ProfileSlotsRestrictions
                    }
                    QueryInfoTypes::q_items_compatibility => QueryClientStatus::ItemsCompatibility,
                    QueryInfoTypes::q_price_items => {
                        if buffer_len != 2 {
                            return Err(DeserializeError::IncorrectInput);
                        }
                        let faction_id = buffer[1];
                        let faction_id = FactionId::from_u8(faction_id)
                            .ok_or(DeserializeError::IncorrectInput)?;
                        QueryClientStatus::PriceItems(faction_id)
                    }
                    QueryInfoTypes::q_account_money => QueryClientStatus::AccountMoney,
                    QueryInfoTypes::q_player_skills => QueryClientStatus::PlayerSkills,
                    QueryInfoTypes::q_player_skills_tree => QueryClientStatus::PlayerSkillsTree,
                    QueryInfoTypes::q_service_prices => QueryClientStatus::ServicePrices,
                    QueryInfoTypes::q_player_reputations => QueryClientStatus::PlayerReputations,
                };

                if !matches!(query_info_type, QueryInfoTypes::q_price_items) {
                    if buffer_len != 4
                        || !(buffer_len == 4 && buffer[1] == 0 && buffer[2] == 0 && buffer[3] == 0)
                    {
                        return Err(DeserializeError::IncorrectInput);
                    }
                }

                Ok(Self::QueryClientStatus(query_client_status))
            }
            lobby_client_message_types_enum::inventory_action => Err(DeserializeError::Todo),
            lobby_client_message_types_enum::shop_action => Err(DeserializeError::Todo),
            lobby_client_message_types_enum::skills_tree_action => Err(DeserializeError::Todo),
            lobby_client_message_types_enum::lobby_client_sign_in_info => match buffer_len {
                4 => {
                    let session_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                    Ok(Self::SignInInfo { session_id })
                }
                _ => Err(DeserializeError::NotEnoughInput),
            },
            lobby_client_message_types_enum::discard_playing_order => Err(DeserializeError::Todo),
            lobby_client_message_types_enum::ping_server => match buffer_len {
                4 => {
                    let current_time = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                    Ok(Self::PingServer { current_time })
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

// 00000000 struct __declspec(align(2)) survarium::price_item // sizeof=0x6
// 00000000 {
// 00000000     unsigned __int16 item_dict_id;
// 00000002     unsigned __int16 cost;
// 00000004     unsigned __int8 reputation_level;
// 00000005     // padding byte
// 00000006 };
