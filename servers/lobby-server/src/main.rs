mod messaging_server;
mod network_client;
mod profile_state;
mod state;

use state::{DeserializeError, FactionId, LobbyClientMessage, QueryClientStatus, QueryInfoTypes};

// use openssl::ssl::{Ssl, SslContext, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;

use foundation::lobby_server;

// const PKEY_PATH: &str = "./certs/survarium_lobby_server.key";
// const CERT_PATH: &str = "./certs/survarium_lobby_server.crt";

const LOCAL_NAME: &str = "sheep";
const ANSWER_NAME: &str = "hello";

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
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
#[allow(non_camel_case_types)]
#[allow(dead_code)]
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

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum messaging_server_message_types_enum {
    messaging_server_connection_successful = 0xC8, // 1
    messaging_server_message               = 0xC9,
    messaging_server_sign_in_info          = 0xCB, // ???
    messaging_friendship_status            = 0xCC,
    messaging_server_invalid_message_type  = 0xFF,
}

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum messaging_client_message_types_enum {
    messaging_client_message              = 0xC1,       // 193
    messaging_client_sign_in_info         = 0xC3, // 1
    messaging_friendship_action           = 0xC4,       // 196
    messaging_client_subscription         = 0xC5, // 2  // 197
    messaging_client_invalid_message_type = 0xC7,
}

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum messaging_client_type_enum {
    unknown_client_type                = 0x0,
    login_server_client_type           = 0x1,
    lobby_server_client_type           = 0x2,
    match_server_client_type           = 0x3,
    message_server_client_type         = 0x4,
    account_client_type                = 0x5,
    administrative_client_type         = 0x6,
    match_maker_server_client_type     = 0x7,
    stats_processor_server_client_type = 0x8,
}

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum messaging_message_channel_enum {
    server_message_channel = 0x0,
    player_general_channel = 0x1,
    player_system_channel  = 0x2,
    player_clan_channel    = 0x3,
    player_private_channel = 0x4,
    player_match_channel   = 0x5,
    player_team1_channel   = 0x6,
    player_team2_channel   = 0x7,
    player_squad_channel   = 0x8,
    max_channel_num        = 0x9,
}

fn main() -> std::io::Result<()> {
    let addr = format!("{}:{}", lobby_server::ADDRESS, lobby_server::PORT);
    let listener = TcpListener::bind(&addr)?;

    for (_, stream) in listener.incoming().enumerate() {
        let stream = stream?;
        std::thread::spawn(move || {
            _ = std::panic::catch_unwind(|| {
                handle_client(stream);
            });
        });
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0_u8; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let buffer = &mut buffer[0..bytes_read].as_ref();
    match LobbyClientMessage::deserialize(buffer) {
        Ok(LobbyClientMessage::SignInInfo { session_id }) => {
            assert!(buffer.is_empty());

            println!("Received **lobby_client_sign_in_info: {session_id}**");
            let (tx, rx) = mpsc::channel::<state::QueryClientStatus>();

            std::thread::spawn({
                let stream = stream.try_clone().unwrap();
                move || {
                    handle_lobby_client_writer(stream, rx);
                }
            });
            handle_lobby_client_reader(stream, tx);
        }
        Ok(msg) => {
            panic!("Received incorrect message. Expected 'SignInInfo': {msg:?}")
        }
        Err(DeserializeError::UnknownMessageType(_)) => {
            let msg_len = buffer[0];
            let msg_type = buffer[1];

            assert_eq!(msg_len, 6);

            if msg_type != messaging_client_message_types_enum::messaging_client_sign_in_info as u8
            {
                panic!("{:?}", buffer);
            }

            let buffer = &buffer[2..];
            let _session_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
            let _unknown_byte = buffer[4];
            // println!("session_id = {session_id}");
            // println!("unknown_byte = {unknown_byte}");
            handle_messaging_client(stream)
        }
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
fn handle_lobby_client_writer(
    mut stream: TcpStream,
    rx: mpsc::Receiver<state::QueryClientStatus>,
) -> ! {
    stream
        .write(&[
            1_u8, // tcp_msg_len (for response)
            lobby_server_message_types_enum::connection_successful as u8,
        ])
        .unwrap();

    loop {
        let msg = rx.recv().unwrap();
        match msg {
            QueryClientStatus::ClientState => {
                // CLIENT STATE
                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(state::QueryInfoTypes::q_client_state as u8);
                buffer.push(0); // m_status
                buffer.push("last_status_message".len() as u8);
                buffer.extend(b"last_status_message");

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **client_state**");
            }
            QueryClientStatus::AccountMoney => {
                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(state::QueryInfoTypes::q_account_money as u8);
                buffer.extend(100_u32.to_le_bytes()); // generic_money
                buffer.extend(10000_u32.to_le_bytes()); // premium_money
                buffer.push(70); // skill_points
                buffer.push(ANSWER_NAME.len() as u8);
                buffer.extend(ANSWER_NAME.as_bytes());

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **account_money**");
            }
            QueryClientStatus::PriceItems(FactionId::Loners) => {
                let ids = [
                    7, 9, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 24, 25, 27, 28, 29, 31, 32, 33,
                    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                    54, 55, 56, 57, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
                ];
                let idx_start = 0;
                let mut buffer = vec![];
                let item_len = ids.len() as u8;

                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(state::QueryInfoTypes::q_price_items as u8);
                buffer.push(FactionId::Loners as u8); // faction_id
                buffer.extend((item_len as u16).to_le_bytes()); // item_len

                for i in idx_start..idx_start + item_len {
                    let i = ids[i as usize];
                    buffer.extend((i as u16).to_le_bytes()); // 1: item_dict_id
                    buffer.extend((i as u16).to_le_bytes()); // 1: cost
                    buffer.extend(0_u8.to_le_bytes()); // 1: reputation_level
                    buffer.extend(0_u8.to_le_bytes()); // 1: padding
                }

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **price_items**");
            }
            QueryClientStatus::PriceItems(_) => {}
            QueryClientStatus::ServicePrices => {
                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(state::QueryInfoTypes::q_service_prices as u8);
                buffer.extend(1_u32.to_le_bytes()); // reroll_cost
                buffer.extend(2_u32.to_le_bytes()); // add_profile_cost
                buffer.extend(3_u32.to_le_bytes()); // rename_account_cost

                network_client::send_message(&mut stream, &buffer);
                println!("[writer] Wrote **service_prices**");
            }
            QueryClientStatus::EnumerateInventory => {
                let mut buffer = vec![];
                buffer.push(lobby_server_message_types_enum::client_status as u8);
                buffer.push(state::QueryInfoTypes::q_enumerate_inventory as u8);

                buffer.extend(1_u32.to_le_bytes()); // num of inventory items

                buffer.extend(
                    profile_state::inventory_item_instance {
                        condition_or_stack: 0,
                        amount_in_inventory: 1,
                        id: 1,
                        dict_id: 55,
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
                buffer.push(state::QueryInfoTypes::q_player_reputations as u8);

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
            _ => (),
        }
    }
}

fn handle_lobby_client_reader(
    mut stream: TcpStream,
    tx: mpsc::Sender<state::QueryClientStatus>,
) -> ! {
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

// [37, 197, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 1, 0, 0,
#[allow(unused_variables)]
fn handle_messaging_client(mut stream: TcpStream) -> ! {
    // Expects:
    // <msg_len> <server_sign_in_info> <local_name>
    // msg_len: local_name.len() | ???
    let mut buffer = vec![];
    buffer.push(LOCAL_NAME.len() as u8 + 2);
    buffer.push(messaging_server_message_types_enum::messaging_server_sign_in_info as u8);
    buffer.push(LOCAL_NAME.len() as u8);
    buffer.extend(LOCAL_NAME.as_bytes());

    let bytes_written = stream.write(&buffer).unwrap();
    // println!("Wrote **messaging_server_sign_in_info**");
    // println!("bytes_written = {bytes_written}");

    //
    //
    //

    loop {
        let mut buffer = [0; 256];
        let bytes_read = stream.read(&mut buffer).unwrap();
        // println!("bytes_read = {bytes_read}");
        // println!("{buffer:?}");

        let mut buffer = vec![];

        let msg = "Hello, friend!";
        let len = 1 + 1 + 4 + 1 + ANSWER_NAME.len() + 1 + 1 + msg.len();

        buffer.push(len as u8);
        buffer.push(messaging_server_message_types_enum::messaging_server_message as u8);
        // client type - 1 byte
        buffer.push(messaging_client_type_enum::lobby_server_client_type as u8);
        // client id - 4 bytes
        buffer.extend(1_u32.to_le_bytes());
        // name len - 1 byte
        buffer.push(ANSWER_NAME.len() as u8);
        // name - name.len() bytes
        buffer.extend(ANSWER_NAME.as_bytes());
        // message channel - 1 byte
        buffer.push(messaging_message_channel_enum::server_message_channel as u8);
        // message len - 1 byte
        buffer.push(msg.len() as u8);
        // message - message.len() bytes
        buffer.extend(msg.as_bytes());

        let bytes_written = stream.write(&buffer).unwrap();
        // println!("Wrote **messaging_server_sign_in_info**");
        // println!("bytes_written = {bytes_written}");
    }
}
