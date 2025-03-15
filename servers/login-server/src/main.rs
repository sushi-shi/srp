use openssl::ssl::{Ssl, SslContext, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use foundation::{browser_server, login_server};

const PKEY_PATH: &str = "./certs/survarium_login_server.key";
const CERT_PATH: &str = "./certs/survarium_login_server.crt";

/// If no prefix is provided the client will construct an incorrect URI like so:
///     "127.0.0.1&type=2&.."
/// Notice that there is no `/` and that the first arguments starts with `&` instead of `?`.
/// As a temporary (?) fix provide an unused argument to the client, since it accepts url prefix
/// as a part of the message.
const URL_PREFIX: &[u8] = b"/hello?unused=1";

/// Hardcoded user session id
const SESSION_ID: u32 = 0x3031;

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum login_server_message_types_enum {
    servers_connection_info_message_type           = 0x8,  // + | logged in | client doesn't start required servers
    password_request_message_type                  = 0x9,  // connection error
    invalid_user_name_or_password_message_type     = 0xA,  // +
    valid_user_name_message_type                   = 0xB,  // + | establishes ssh connection
    sign_in_attempt_interval_violated_message_type = 0xC,  // +
    sign_out_successful                            = 0xD,  // connection error
    occupied_user_name_message_type                = 0xE,  // connection error
    send_sign_up_info_message_type                 = 0xF,  // connection error
    sign_up_successful_message_type                = 0x10, // connection error
    user_banned_message_type                       = 0x11, // +
    user_restricted_by_access_level_message_type   = 0x12, // +
    sign_in_user_already_signed_in                 = 0x13, // + | got stuck on connecting
    sign_in_invalid_version                        = 0x14, // +
    login_server_invalid_message_type              = 0x1F, // connection error
}

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum login_client_message_types_enum {
    sign_up_message_type              = 0x0, // not used by the client
    sign_in_message_type              = 0x1,
    sign_out_message_type             = 0x2,
    login_client_invalid_message_type = 0x7, // not used by the client
}

/// @NOTE: Real solution should:
/// * Accept the config for addresses of other servers
/// * Use `async` instead of threads
/// * Have a database for different users and their session ids
/// * ...many more things
fn main() -> std::io::Result<()> {
    let addr = format!("{}:{}", login_server::ADDRESS, login_server::PORT);
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
        _ if request_type == login_client_message_types_enum::sign_in_message_type as u8 => {
            handle_sign_in(stream);
        }
        _ if request_type == login_client_message_types_enum::sign_out_message_type as u8 => {
            handle_sign_out(stream);
        }
        _ => unreachable!("Unknown request type: {request_type}"),
    }
}

fn handle_sign_in(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("bytes_read = {bytes_read}");

    let email_len = buffer[0] as usize;
    let email = buffer[1..1 + email_len].to_vec();
    let game_version = buffer[1 + email_len..1 + email_len + 6].to_vec();

    let email = String::from_utf8(email).unwrap();
    let game_version = String::from_utf8(game_version).unwrap();

    println!("User email: {email}");
    println!("Client version: {game_version}");

    let message_type = login_server_message_types_enum::valid_user_name_message_type as u8;
    stream.write(&[message_type]).unwrap();

    match () {
        _ if message_type
            == login_server_message_types_enum::valid_user_name_message_type as u8 =>
        {
            println!("    ****valid_user_name_message_type****    ");
            let mut builder = SslContext::builder(SslMethod::tls()).unwrap();
            builder.set_security_level(0);
            builder
                .set_private_key_file(PKEY_PATH, SslFiletype::PEM)
                .unwrap();
            builder.set_certificate_chain_file(CERT_PATH).unwrap();
            let context = builder.build();
            let ssl = Ssl::new(&context).unwrap();

            let mut ssl_stream = ssl.accept(stream).unwrap();
            let mut buffer = [0; 1024];
            ssl_stream.read(&mut buffer).unwrap();

            let password_len = buffer[0] as usize;
            let password = buffer[1..1 + password_len].to_vec();
            let password = String::from_utf8(password).unwrap();

            println!("User password: {password}");

            let mut buffer = vec![];
            buffer
                .push(login_server_message_types_enum::servers_connection_info_message_type as u8);

            buffer.push(browser_server::ADDRESS.len() as u8);
            buffer.extend(browser_server::ADDRESS.as_bytes());
            buffer.push(URL_PREFIX.len() as u8);
            buffer.extend(URL_PREFIX);
            buffer.extend(SESSION_ID.to_le_bytes());

            ssl_stream.write(&buffer).unwrap();
        }
        // Support for incorrect login, user being banned, etc.
        // See: `login_server_message_types_enum`
        _ => unreachable!(),
    }
}

// @TODO: Properly parse the given buffer
// bytes_read = 214
// [32, 20, 0, 0, 22, 3, 1, 0, 205, 1, 0, 0, 201, 3, 1, 103, 207, 95, 149, 60, 132, 54, 212, 101, 56, 90, 184, 201, 216, 166, 73, 57, 2, 61, 115, 172, 214, 81, 233, 148, 39, 183, 176, 167, 233, 210, 113, 0, 0, 92, 192, 20, 192, 10, 0, 57, 0, 56, 0, 136, 0, 135, 192, 15, 192, 5, 0, 53, 0, 132, 192, 18, 192, 8, 0, 22, 0, 19, 192, 13, 192, 3, 0, 10, 192, 19, 192, 9, 0, 51, 0, 50, 0, 154, 0, 153, 0, 69, 0, 68, 192, 14, 192, 4, 0, 47, 0, 150, 0, 65, 0, 7, 192, 17, 192, 7, 192, 12, 192, 2, 0, 5, 0, 4, 0, 21, 0, 18, 0, 9, 0, 20, 0, 17, 0, 8, 0, 6, 0, 3, 0, 255, 1, 0, 0, 68, 0, 11, 0, 4, 3, 0, 1, 2, 0, 10, 0, 52, 0, 50, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 19, 0, 20, 0, 21, 0, 22, 0, 23, 0, 24, 0, 25, 0, 35, ... ]
// [32, 20, 0, 0, 22, 3, 1, 0, 205, 1, 0, 0, 201, 3, 1, 103, 207, 94, 162, 152, 29, 237, 244, 33, 121, 81, 253, 106, 28, 14, 35, 203, 222, 76, 221, 244, 181, 15, 33, 191, 218, 24, 225, 122, 231, 11, 254, 0, 0, 92, 192, 20, 192, 10, 0, 57, 0, 56, 0, 136, 0, 135, 192, 15, 192, 5, 0, 53, 0, 132, 192, 18, 192, 8, 0, 22, 0, 19, 192, 13, 192, 3, 0, 10, 192, 19, 192, 9, 0, 51, 0, 50, 0, 154, 0, 153, 0, 69, 0, 68, 192, 14, 192, 4, 0, 47, 0, 150, 0, 65, 0, 7, 192, 17, 192, 7, 192, 12, 192, 2, 0, 5, 0, 4, 0, 21, 0, 18, 0, 9, 0, 20, 0, 17, 0, 8, 0, 6, 0, 3, 0, 255, 1, 0, 0, 68, 0, 11, 0, 4, 3, 0, 1, 2, 0, 10, 0, 52, 0, 50, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 19, 0, 20, 0, 21, 0, 22, 0, 23, 0, 24, 0, 25, 0, 35, ... ]
// | session_id |                                                  x                                                                                                                                        x + 30                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           x
fn handle_sign_out(mut stream: TcpStream) {
    let mut buffer = [0; 256];
    let bytes_read = stream.read(&mut buffer).unwrap();

    println!("bytes_read = {bytes_read}");
    println!("{buffer:?}");

    let buffer: [u8; 4] = buffer[0..4].try_into().unwrap();
    let session_id = u32::from_le_bytes(buffer);
    println!("session_id = {session_id}");
    let string = buffer[4..].to_vec();
    let string = String::from_utf8_lossy(&string);
    println!("string = {string}");

    //
    //
    //

    let mut builder = SslContext::builder(SslMethod::tls()).unwrap();
    builder.set_security_level(0);
    builder
        .set_private_key_file(PKEY_PATH, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(CERT_PATH).unwrap();
    let context = builder.build();
    let ssl = Ssl::new(&context).unwrap();

    let mut ssl_stream = ssl.accept(stream).unwrap();
    let mut buffer = [0; 1024];
    ssl_stream.read(&mut buffer).unwrap();

    let password_len = buffer[0] as usize;
    let password = buffer[1..1 + password_len].to_vec();
    let password = String::from_utf8(password).unwrap();

    println!("User password: {password}");
}
