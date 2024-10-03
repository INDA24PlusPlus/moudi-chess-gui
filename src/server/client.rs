use std::{io::{ErrorKind, Read, Write}, mem, net::{SocketAddr, TcpStream}};

use chess_networking::Start;

pub struct ServerClient {
    stream: TcpStream,
    addr: SocketAddr,
    opts: Option<Start>,
}

impl ServerClient {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            stream,
            addr,
            opts: None
        }
    }

    pub fn fill(&mut self, opts: Start) -> Self {
        self.opts = Some(opts.clone());
        Self {
            stream: self.stream.try_clone().unwrap(),
            addr: self.addr,
            opts: Some(opts),
        }
    }

    pub fn is_alive(&self) -> bool {
        let mut buf = [0u8; 1];
        match self.stream.peek(&mut buf) {
            Ok(0) => false,
            Ok(_) => true,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => true,
            Err(_) => false,
        }
    }

    pub fn is_established(&self) -> bool {
        self.opts.is_some()
    }
}

impl ServerClient {
    pub fn read_start(&mut self) -> Option<Start> {
        let mut buf = [0u8; mem::size_of::<Start>()];
        let _ = self.stream.read(&mut buf);

        match chess_networking::Start::try_from(buf.as_slice()) {
            Ok(start) => {
                println!("Successfully read start");
                Some(start)
            },
            Err(e) => {
                println!("Error(START PACKET): {}", e);
                None
            }
        }
    }

    pub fn send_start(&mut self) {
        if let Some(start) = &self.opts {
            let buf : Vec<u8> = start.clone().try_into().unwrap();
            if let Err(e) = self.stream.write_all(&buf) {
                println!("\nError sending start: {}\n", e);
            }
        }
    }
}
