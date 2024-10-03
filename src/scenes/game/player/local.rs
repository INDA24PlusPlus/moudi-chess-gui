use raylib::prelude::*;
use crate::scenes::game::screen::screen_to_board_coord;

use super::Player;

pub struct LocalPlayer {
    selected: Option<i32>,
    move_to: Option<i32>,
    mouse: (i32, i32),
}

impl Player for LocalPlayer {
    fn on_ongoing(&mut self, rl: &mut RaylibHandle) {
        self.mouse = (rl.get_mouse_x(), rl.get_mouse_y());
        if !rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            return;
        }

        let (x, y) = screen_to_board_coord(self.mouse.0, self.mouse.1);

        if !(0..8).contains(&x) || !(0..8).contains(&y) {
            return;
        }

        if let Some(selected_index) = self.selected {
            if selected_index != y * 8 + x {
                self.move_to = Some(y * 8 + x);
            } else {
                self.on_move_piece();
            }
        } else {
            self.selected = Some(y * 8 + x);
        }
    }

    fn on_promotion(&mut self) -> bool {true}
    fn on_end(&mut self) {
        println!("end");
    }

    fn on_move_piece(&mut self) -> bool {
        self.clear_selected();
        self.move_to = None;
        true
    }

    fn get_move(&self) -> Option<i32> {
        self.move_to
    }

    fn get_selected_slot(&self) -> Option<i32> {
        self.selected
    }

    fn get_mouse(&self) -> (i32, i32) {
        self.mouse
    }

    fn clear_selected(&mut self) {
        self.selected = None;
    }

}

impl LocalPlayer {
    pub fn init() -> Self {
        Self {
            selected: None,
            move_to: None,
            mouse: (0, 0),
        }
    }
}
