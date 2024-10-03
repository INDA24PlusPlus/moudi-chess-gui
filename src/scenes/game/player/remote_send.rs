
use raylib::prelude::*;

use super::Player;

pub struct RemoteSendPlayer { }

impl Player for RemoteSendPlayer { 
    fn on_ongoing(&mut self, rl: &mut RaylibHandle) { }
    fn on_promotion(&mut self) -> bool { false }
    fn on_end(&mut self) { }
    fn on_move_piece(&mut self) -> bool {
        false
    }

    fn get_move(&self) -> Option<i32> { None }
    fn get_selected_slot(&self) -> Option<i32> { None }
    fn get_mouse(&self) -> (i32, i32) { (-1, -1) }
    fn clear_selected(&mut self) { }
}

impl RemoteSendPlayer {
    pub fn init() -> Self {
        RemoteSendPlayer {}
    }
}
