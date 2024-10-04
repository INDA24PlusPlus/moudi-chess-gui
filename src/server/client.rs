use std::{fmt::{Debug}, io::{ErrorKind, Read, Write, Error}, mem, net::{SocketAddr, TcpStream}};

use chess_networking::{Ack, Move, Start};

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
        let mut buf = [0u8; 1024];
        let _ = self.stream.read(&mut buf);
        
        match chess_networking::Start::try_from(buf.as_slice()) {
            Ok(start) => {
                println!("Successfully read start");
                Some(start)
            },
            Err(e) => {
                println!("Server Error: {}", e);
                None
            }
        }
    }

    pub fn send_start(&mut self) {
        if let Some(start) = &self.opts {
            if let Err(e) = send(&mut self.stream, start) {
                println!("\nServer Error sending start: {}\n", e);
            }
        }
    }

    pub fn read_move(&mut self) -> Option<Move> {
        // let mut buf = [0u8; mem::size_of::<Move>()];
        let mut buf = [0u8; 1024];
        let _ = self.stream.read(&mut buf);

        match chess_networking::Move::try_from(buf.as_slice()) {
            Ok(_move) => {
                println!("Successfully read move");
                Some(_move)
            },
            Err(rmp_serde::decode::Error::Syntax(_)) => { None },
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    pub fn send_move(&mut self, _move: &Move) {
        if let Err(e) = send(&mut self.stream, _move) {
            println!("\nServer Error sending move: {}\n", e);
        }
    }

    pub fn read_ack(&mut self) -> Option<Ack> {
        let mut buf = [0u8; 1024];
        let _ = self.stream.read(&mut buf);

        match Ack::try_from(&buf[..]) {
            Ok(ack) => {
                Some(ack)
            },
            Err(rmp_serde::decode::Error::Syntax(_)) => { None },
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    pub fn send_ack(&mut self, ack: Ack) {
        if let Err(e) = send(&mut self.stream, &ack) {
            println!("\nServer Error sending ack: {}\n", e);
        }
    }
}

pub fn send<T>(stream: &mut TcpStream, value: &T) -> Result<(), Error>
    where T: TryInto<Vec<u8>> + Clone,
          T::Error: Debug
{
    let buf : Vec<u8> = value.clone().try_into().unwrap();
    stream.write_all(&buf)
}
