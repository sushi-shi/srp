#![allow(dead_code)] // Ssl, certificats and session ids will be used later on
#![allow(unused_imports)]

mod messaging_server;

use openssl::ssl::{Ssl, SslContext, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use foundation::lobby_server;

const PKEY_PATH: &str = "./certs/survarium_lobby_server.key";
const CERT_PATH: &str = "./certs/survarium_lobby_server.crt";

/// Hardcoded user session id
const SESSION_ID: u32 = 0x3031;
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
    connect_to_match_server           = 0x33, // ??
    operation_permitted               = 0x34, // ??
    operation_denied                  = 0x35, // ??
    client_status                     = 0x36, // ??
    ping_server_answer                = 0x37,
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
    let mut msg_len = [0_u8];
    stream.read(&mut msg_len).unwrap();
    let msg_len = msg_len[0] - 1;

    let mut request_type = [0_u8];
    stream.read(&mut request_type).unwrap();
    let request_type = request_type[0];
    println!("Received message of type: {request_type}");

    match request_type <= lobby_client_message_types_enum::lobby_client_invalid_message_type as u8 {
        true => handle_lobby_client(stream, request_type, msg_len),
        false => handle_messaging_client(stream, request_type, msg_len),
    }
}

fn handle_lobby_client(mut stream: TcpStream, request_type: u8, msg_len: u8) {
    match () {
        _ if request_type == lobby_client_message_types_enum::lobby_client_sign_in_info as u8 => {
            assert_eq!(msg_len, 4);
            let mut buffer = [0_u8; 4];
            stream.read_exact(&mut buffer).unwrap();
            let session_id = u32::from_le_bytes(buffer);
            println!("session_id = {session_id}");

            stream
                .write(&[lobby_server_message_types_enum::connection_successful as u8])
                .unwrap();

            //
            //
            //

            let mut buffer = [0; 256];
            let bytes_read = stream.read(&mut buffer).unwrap();
            println!("bytes_read = {bytes_read}");
            println!("{buffer:?}");

            // let mut builder = SslContext::builder(SslMethod::tls()).unwrap();
            // builder.set_security_level(0);
            // builder
            //     .set_private_key_file(PKEY_PATH, SslFiletype::PEM)
            //     .unwrap();
            // builder.set_certificate_chain_file(CERT_PATH).unwrap();
            // let context = builder.build();
            // let ssl = Ssl::new(&context).unwrap();

            // let mut ssl_stream = ssl.accept(stream).unwrap();
            // let mut buffer = [0; 256];
            // let bytes_read = ssl_stream.read(&mut buffer).unwrap();
            // println!("bytes_read = {bytes_read}");
            // println!("{buffer:?}");
        }
        _ => unreachable!("Unknown lobby request type: {request_type}"),
    }
}

// [37, 197, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 1, 0, 0,
fn handle_messaging_client(mut stream: TcpStream, request_type: u8, msg_len: u8) -> ! {
    match () {
        _ if request_type
            == messaging_client_message_types_enum::messaging_client_sign_in_info as u8 =>
        {
            assert_eq!(msg_len, 5);
            let mut buffer = [0_u8; 5];
            stream.read_exact(&mut buffer).unwrap();
            let session_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
            println!("session_id = {session_id}");

            let unknown_byte = buffer[4];
            println!("unknown_byte = {unknown_byte}");

            //
            //
            //

            // Expects:
            // <msg_len> <server_sign_in_info> <local_name>
            // msg_len: local_name.len() | ???
            let mut buffer = vec![];
            buffer.push(LOCAL_NAME.len() as u8 + 2);
            buffer.push(messaging_server_message_types_enum::messaging_server_sign_in_info as u8);
            buffer.push(LOCAL_NAME.len() as u8);
            buffer.extend(LOCAL_NAME.as_bytes());

            let bytes_written = stream.write(&buffer).unwrap();
            println!("Wrote **messaging_server_sign_in_info**");
            println!("bytes_written = {bytes_written}");

            //
            //
            //

            loop {
                let mut buffer = [0; 256];
                let bytes_read = stream.read(&mut buffer).unwrap();
                println!("bytes_read = {bytes_read}");
                println!("{buffer:?}");

                let mut buffer = vec![];

                let msg = "Hello, friend!";
                let len = msg.len() + ANSWER_NAME.len() + 4 + 2 + 1;

                buffer.push(len as u8);
                buffer.push(messaging_server_message_types_enum::messaging_server_message as u8);
                buffer.extend(1_u32.to_le_bytes());
                buffer.push(ANSWER_NAME.len() as u8);
                buffer.extend(ANSWER_NAME.as_bytes());
                buffer.push(msg.len() as u8);
                buffer.extend(msg.as_bytes());

                let bytes_written = stream.write(&buffer).unwrap();
                println!("Wrote **messaging_server_sign_in_info**");
                println!("bytes_written = {bytes_written}");
            }
        }
        _ => unreachable!("Unknown messaging request type: {request_type}"),
    }
}

// Received message of type: 5
// bytes_read = 5
// [38, 49, 48, 0, 0]
// Received message of type: 6
// bytes_read = 6
// [195, 49, 48, 0, 0, 5]
fn handle_five(mut stream: TcpStream) {
    let mut buffer = [0; 256];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("bytes_read = {bytes_read}");
    // [38, 196, 9, 0, 0, ..]
    //      session_id
    println!("{:?}", buffer);

    stream
        .write(&[lobby_server_message_types_enum::connection_successful as u8])
        .unwrap();
    stream.write(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]).unwrap();
    stream.write(&buffer).unwrap();

    // ???
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("bytes_read = {bytes_read}");
    println!("{:?}", buffer);
    std::thread::sleep(std::time::Duration::from_secs(2));
}
