mod lobby_server;
mod messaging_server;
mod network_client;

use lobby_server::LobbyClientMessage;

use std::io::Read;
use std::net::{TcpListener, TcpStream};

const LOCAL_NAME: &str = "sheep";
const ANSWER_NAME: &str = "hello";

fn main() -> std::io::Result<()> {
    let addr = format!(
        "{}:{}",
        foundation::lobby_server::ADDRESS,
        foundation::lobby_server::PORT
    );
    let listener = TcpListener::bind(&addr)?;

    for (_, stream) in listener.incoming().enumerate() {
        let stream = stream?;
        std::thread::spawn(move || {
            _ = std::panic::catch_unwind(|| {
                handle_connection(stream);
            });
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0_u8; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let buffer = &buffer[0..bytes_read];

    match () {
        () if LobbyClientMessage::is_lobby_client_message(buffer) => {
            lobby_server::handle(stream, &buffer)
        }
        () => messaging_server::handle(stream, &buffer),
    }
}
