use std::io::Write;
use std::net::TcpStream;

pub struct Packet {
    buffer: Vec<u8>,
}
impl Extend<u8> for Packet {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        self.buffer.extend(iter)
    }
}

impl<'a> Extend<&'a u8> for Packet {
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) {
        self.buffer.extend(iter)
    }
}

impl Packet {
    pub fn new() -> Self {
        let mut buffer = Vec::with_capacity(128);
        buffer.push(0);
        buffer.push(0);
        buffer.push(0);
        Self { buffer }
    }

    pub fn push(&mut self, byte: u8) {
        self.buffer.push(byte)
    }

    pub fn send(mut self, tcp_stream: &mut TcpStream) {
        let msg_len = self.buffer.len() - 3;

        let buffer = if msg_len >= 0x100 {
            let msg_len: u16 = msg_len.try_into().expect("All messages must fit into u16");
            let msg_len = msg_len.to_le_bytes();
            self.buffer[1] = msg_len[0];
            self.buffer[2] = msg_len[1];

            self.buffer.as_ref()
        } else {
            let msg_len: u8 = msg_len.try_into().expect("Checked in if");
            self.buffer[2] = msg_len;

            self.buffer[2..].as_ref()
        };

        tcp_stream.write_all(buffer).unwrap();
    }
}
