
use std::{io::{Read, Write}, mem, net::TcpStream};

use chess_networking::{Ack, Move};
use raylib::prelude::*;

use super::{local::LocalPlayer, Player};

pub struct RemoteSendPlayer {
    local: LocalPlayer,
    stream: TcpStream
}

impl Player for RemoteSendPlayer { 
    fn on_ongoing(&mut self, rl: &mut RaylibHandle) {
        self.local.on_ongoing(rl);
    }
    fn on_promotion(&mut self) -> bool { false }
    fn on_end(&mut self) { }
    fn on_move_piece(&mut self) -> bool {
        if let (Some(from), Some(to)) = (self.get_selected_slot(), self.get_move()) {
            let move_obj = Move {
                from: (from as u8 % 8, from as u8 / 8),
                to: (to as u8 % 8, to as u8 / 8),
                promotion: None,
                offer_draw: false,
                forfeit: false,
            };

            println!("Sending move to server");
            {
                let buf : Vec<u8> = move_obj.try_into().unwrap();
                self.stream.write_all(&buf).unwrap();
            }
            println!("Sent success!");

            println!("Watiting for ack from server");
            {
                let mut buf = [0u8 ; mem::size_of::<Ack>()];
                let _ = self.stream.set_nonblocking(false);
                loop {
                    if self.stream.read(&mut buf).unwrap() != 0 {
                        break;
                    }
                }
                let _ = self.stream.set_nonblocking(true);
            }
            println!("Ack recieved!");
            self.local.on_move_piece();
            return true;
        }

        false
    }

    fn get_move(&self) -> Option<i32> {
        self.local.get_move()
    }
    fn get_selected_slot(&self) -> Option<i32> {
        self.local.get_selected_slot()
    }
    fn get_mouse(&self) -> (i32, i32) { 
        self.local.get_mouse()
    }
    fn clear_selected(&mut self) {
        self.local.clear_selected();
    }
}

impl RemoteSendPlayer {
    pub fn init(stream: TcpStream) -> Self {
        let _ = stream.set_nonblocking(true);
        RemoteSendPlayer {
            local: LocalPlayer::init(),
            stream
        }
    }
}
