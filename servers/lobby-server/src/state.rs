use num_traits::FromPrimitive;

#[rustfmt::skip]
#[derive(num_derive::FromPrimitive)]
#[allow(non_camel_case_types)]
#[repr(u8)]
enum lobby_client_message_types_enum {
    set_status_ready_for_match        = 0x20,
    query_client_status               = 0x21, // 3
    inventory_action                  = 0x23,
    shop_action                       = 0x24,
    skills_tree_action                = 0x25,
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

// [5, 33, 5 , 0, 0, 0,
// [5, 33, 9 , 0, 0, 0,
// [5, 33, 10, 0, 0, 0,
// [3, 33, 6, 1,
// [3, 33, 6, 2,
// [3, 33, 6, 3,
// [3, 33, 6, 4,
// [5, 33, 0 , 0, 0, 0,
#[derive(Debug, PartialEq)]
pub enum QueryClientStatus {
    QueryClientStatus {
        unknown_type: u32,
    },
    QueryPrices {
        unknown_type_1: u8,
        unknown_type_2: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DeserializeError {
    NotEnoughInput,
    UnknownMessageType(u8),
    Todo,
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
        use lobby_client_message_types_enum::*;
        let result = match msg_type {
            set_status_ready_for_match => Err(DeserializeError::Todo),
            query_client_status => match buffer_len {
                2 => Ok(Self::QueryClientStatus(QueryClientStatus::QueryPrices {
                    unknown_type_1: buffer[0],
                    unknown_type_2: buffer[1],
                })),
                4 => {
                    let unknown_type = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                    Ok(Self::QueryClientStatus(
                        QueryClientStatus::QueryClientStatus { unknown_type },
                    ))
                }
                _ => Err(DeserializeError::NotEnoughInput),
            },
            inventory_action => Err(DeserializeError::Todo),
            shop_action => Err(DeserializeError::Todo),
            skills_tree_action => Err(DeserializeError::Todo),
            lobby_client_sign_in_info => match buffer_len {
                4 => {
                    let session_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                    Ok(Self::SignInInfo { session_id })
                }
                _ => Err(DeserializeError::NotEnoughInput),
            },
            discard_playing_order => Err(DeserializeError::Todo),
            ping_server => match buffer_len {
                4 => {
                    let current_time = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
                    Ok(Self::PingServer { current_time })
                }
                _ => Err(DeserializeError::NotEnoughInput),
            },
            lobby_client_invalid_message_type => Err(DeserializeError::Todo),
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
    let dejure = LobbyClientMessage::QueryClientStatus(QueryClientStatus::QueryClientStatus {
        unknown_type: 10,
    });
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
        LobbyClientMessage::QueryClientStatus(QueryClientStatus::QueryClientStatus {
            unknown_type: 10,
        }),
    );

    assert_eq!(
        msgs[4],
        LobbyClientMessage::QueryClientStatus(QueryClientStatus::QueryPrices {
            unknown_type_1: 6,
            unknown_type_2: 2
        }),
    );

    assert_eq!(
        msgs[7],
        LobbyClientMessage::QueryClientStatus(QueryClientStatus::QueryClientStatus {
            unknown_type: 0,
        }),
    );
}
