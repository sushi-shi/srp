use openssl::ssl::{Ssl, SslContext, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const PKEY_PATH: &str = "./certs/survarium_login_server.key";
const CERT_PATH: &str = "./certs/survarium_login_server.crt";

// XREF: boost::_bi::value<enum vostok::login_server_message_types_enum>/r
// ?invoke@?$void_function_invoker4@P6AXW4connection_error_types_enum@vostok@@W4handshaking_error_types_enum@2@W4socket_error_types_enum@2@W4login_server_message_types_enum@2@@ZXW412@W432@W442@W452@@function@detail@boost@@SAXAATfunction_buffer@234@W4connection_error_types_enum@vostok@@W4handshaking_error_types_enum@7@W4socket_error_types_enum@7@W4login_server_message_types_enum@7@@Z/r ...
#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum login_server_message_types_enum {
    servers_connection_info_message_type           = 0x8,  // + | logged in
    password_request_message_type                  = 0x9,  // connection error
    invalid_user_name_or_password_message_type     = 0xA,  // +
    valid_user_name_message_type                   = 0xB,  // ssh + attempt interval violated
    sign_in_attempt_interval_violated_message_type = 0xC,  // attempt interval violated
    sign_out_successful                            = 0xD,  // connection error
    occupied_user_name_message_type                = 0xE,  // connection error
    send_sign_up_info_message_type                 = 0xF,  // connection error
    sign_up_successful_message_type                = 0x10, // connection error
    user_banned_message_type                       = 0x11, // +
    user_restricted_by_access_level_message_type   = 0x12, // + | access level restriction
    sign_in_user_already_signed_in                 = 0x13, // + | got stuck on connecting
    sign_in_invalid_version                        = 0x14, // +
    login_server_invalid_message_type              = 0x1F, // connection error
}

#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum login_client_message_types_enum {
    sign_up_message_type              = 0x0,
    sign_in_message_type              = 0x1,
    sign_out_message_type             = 0x2,
    login_client_invalid_message_type = 0x7,
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;

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
    println!("Received message of type: {n}", n = request_type[0]);

    let request_type = request_type[0];
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
            == login_server_message_types_enum::servers_connection_info_message_type as u8 =>
        {
            println!("    ****servers_connection_info_message_type****    ");
            // The server closes the connection with us
            return;
        }
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

            // Figure out what exactly the server expected with
            // `servers_connection_info_message_type`.
            // Currently the URL is broken and it seems like instead of actual URL it reads garbage
            // from old buffers.

            // We need to understand how this buffer is initialized.
            // It seems like it is initialized incorrectly, which is causing UB, which we then notice.
            //
            // char *__thiscall vostok::network::login_client::server_browser_address(vostok::network::login_client *this)
            // {
            //      return this->m_client->m_server_browser_address;
            // }
            //
            // See what exactly server expects when creating a SSL connection
            //
            // This is `login_client_impl`. Check all places where `m_server_browser_address` is written.

            const SERVER_BYTES: &[u8] = b"survarium.com";

            let mut buffer = vec![];
            buffer
                .push(login_server_message_types_enum::servers_connection_info_message_type as u8);

            buffer.push(SERVER_BYTES.len() as u8);
            buffer.extend(SERVER_BYTES);
            buffer.push(0);
            buffer.extend(5152_u32.to_le_bytes());

            ssl_stream.write(&buffer).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(2));
        }
        _ => {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}

fn handle_sign_out(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    println!("bytes_read = {bytes_read}");
    println!("{:?}", buffer);

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

    std::thread::sleep(std::time::Duration::from_secs(2));
}
