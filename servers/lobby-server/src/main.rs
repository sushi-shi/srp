#![allow(dead_code)] // Ssl, certificats and session ids will be used later on
#![allow(unused_imports)]

use openssl::ssl::{Ssl, SslContext, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use foundation::lobby_server;

const PKEY_PATH: &str = "./certs/survarium_lobby_server.key";
const CERT_PATH: &str = "./certs/survarium_lobby_server.crt";

/// Hardcoded user session id
const SESSION_ID: u32 = 0x3031;

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum lobby_server_message_types_enum {
    connection_successful             = 0x30,
    invalid_session_id                = 0x31,
    invalid_password                  = 0x32,
    connect_to_match_server           = 0x33,
    operation_permitted               = 0x34,
    operation_denied                  = 0x35,
    client_status                     = 0x36,
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
    lobby_client_sign_in_info         = 0x26,
    discard_playing_order             = 0x27,
    ping_server                       = 0x28,
    lobby_client_invalid_message_type = 0x2F,
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
    let mut request_type = [0_u8];
    stream.read(&mut request_type).unwrap();

    let request_type = request_type[0];
    println!("Received message of type: {request_type}");

    match () {
        _ if request_type == 5 || request_type == 6 => {
            handle_five(stream);
        }
        _ => unreachable!("Unknown request type: {request_type}"),
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
