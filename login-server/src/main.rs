use openssl::ssl::{Ssl, SslContext, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const PKEY_PATH: &str = "./certs/survarium_login_server.key";
const CERT_PATH: &str = "./certs/survarium_login_server.crt";

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{buffer:?}");

    let proto_version = buffer[0];
    let email_len = buffer[1] as usize;
    let email = buffer[2..2 + email_len].to_vec();
    let game_version = buffer[2 + email_len..2 + email_len + 6].to_vec();

    let email = String::from_utf8(email).unwrap();
    let game_version = String::from_utf8(game_version).unwrap();

    assert_eq!(proto_version, 1);
    println!("User email: {email}");
    println!("Client version: {game_version}");

    let msg = &[11u8];
    stream.write(msg).unwrap();

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

    println!("{buffer:?}");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
