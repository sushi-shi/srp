use std::io::Write;
use std::net::TcpStream;

// @NOTE: This will always allocate, optimize this if ever becomes a problem.
// Survarium actually does this already, cool
pub fn send_message(tcp_stream: &mut TcpStream, msg: &[u8]) {
    let mut buffer = vec![];

    if msg.len() >= 0x100 {
        buffer.push(0_u8);
        buffer.extend((msg.len() as u16).to_le_bytes());
    } else {
        buffer.push(msg.len() as u8);
    }

    buffer.extend(msg);

    tcp_stream.write_all(buffer.as_ref()).unwrap();
}
