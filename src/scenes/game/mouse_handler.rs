use raylib::{ffi::MouseButton, RaylibHandle};
use viktoe_chess::{board::Turn, piece::Color, prelude::BoardPosition};

use crate::scenes::{game::screen::screen_to_board_coord, SceneStorage};


pub fn update_mouse_action(rl: &mut RaylibHandle, scene: &mut SceneStorage) {
    scene.game.mouse = rl.get_mouse_position();
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let (x, y) = screen_to_board_coord(rl.get_mouse_x(), rl.get_mouse_y());
        let clicked_index = y * 8 + x;

        // check if clicked index was already selected
        if let Some(selected_index) = scene.game.selected_slot {
            if selected_index != clicked_index {
               let _ = scene.chess.move_piece(
                    &BoardPosition::try_from(((selected_index % 8) as u8, (selected_index / 8) as u8)).unwrap(), 
                    &BoardPosition::try_from((x as u8, y as u8)).unwrap());

            }
            scene.game.selected_slot = None;
            scene.game.selected_piece_moves = None;
        } else {
            let pos = &BoardPosition::try_from((x as u8, y as u8)).unwrap();
            if let Some(piece) = scene.chess.get_square(pos) {
                let turn = scene.chess.get_player_turn();

                // if this piece is on the side of the players turn
                if (matches!(piece, Color::White(_)) && matches!(turn, Turn::White)) || (matches!(piece, Color::Black(_)) && matches!(turn, Turn::Black)) {
                    scene.game.selected_piece_moves = scene.chess.get_valid_moves(pos);
                    scene.game.selected_slot = Some(clicked_index);
                }
            }
        }
    }
}
