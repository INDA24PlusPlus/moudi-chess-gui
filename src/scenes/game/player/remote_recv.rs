use std::{io::{self, ErrorKind, Read}, net::TcpStream};

use chess_networking::{Ack, Move};
use raylib::RaylibHandle;

use super::Player;

pub struct RemoteRecvPlayer {
    from: Option<i32>,
    to: Option<i32>,
    attempted_move: bool,
    stream: TcpStream
}

impl Player for RemoteRecvPlayer { 
    fn on_ongoing(&mut self, rl: &mut RaylibHandle) {
        if self.attempted_move {
            println!("ack[ ok: {} ]", !(self.from.is_some() && self.to.is_some()));
            self.attempted_move = false;
        }


        let mut buf = [0u8; 1024];
        match self.stream.read(&mut buf) {
            Ok(size) => {
                if size == 0 {
                    return;
                }
            },

            Err(e) => {
                println!("Error(RecvPlayer ongoing): {}", e);
                return;
            }
        }
        
        if let Ok(_move) = Move::try_from(&buf[..]) {
            self.from = Some(_move.from.1 as i32 * 8 + _move.from.0 as i32);
            self.to = Some(_move.to.1 as i32 * 8 + _move.to.0 as i32);
            self.attempted_move = true;
        }
    }
    fn on_promotion(&mut self) -> bool { false }
    fn on_end(&mut self) { }
    fn on_move_piece(&mut self) -> bool {
        self.clear_selected();
        self.to = None;
        true
    }

    fn get_move(&self) -> Option<i32> {
        self.to
    }
    
    fn get_selected_slot(&self) -> Option<i32> {
        self.from
    }

    fn get_mouse(&self) -> (i32, i32) { (-1, -1) }
    fn clear_selected(&mut self) {
        self.from = None;
    }
}

impl RemoteRecvPlayer {
    pub fn init(stream: TcpStream) -> Self {
        let _ = stream.set_nonblocking(true);
        RemoteRecvPlayer {
            from: None,
            to: None,
            attempted_move: false,
            stream
        }
    }
}
