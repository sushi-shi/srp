use super::{ANSWER_NAME, LOCAL_NAME};

use std::io::{Read, Write};
use std::net::TcpStream;

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

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum friendship_actions_enum {
    add_friend            = 0x0,
    remove_friend         = 0x1,
    add_ignorable         = 0x2,
    remove_ignorable      = 0x3,
    find_players          = 0x4,
    query_friend_list     = 0x5,
    query_ignore_list     = 0x6,
    update_friends_status = 0x7,
}

pub fn handle(stream: TcpStream, buffer: &[u8]) -> ! {
    let msg_len = buffer[0];
    let msg_type = buffer[1];

    assert_eq!(msg_len, 6);

    if msg_type != messaging_client_message_types_enum::messaging_client_sign_in_info as u8 {
        panic!("{:?}", buffer);
    }

    let buffer = &buffer[2..];
    let _session_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
    let _unknown_byte = buffer[4];
    // println!("session_id = {session_id}");
    // println!("unknown_byte = {unknown_byte}");

    run(stream)
}

// [37, 197, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 1, 0, 0,
#[expect(unused_variables)]
fn run(mut stream: TcpStream) -> ! {
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
