use std::borrow::{Borrow, BorrowMut};

use raylib::{ffi::MouseButton, RaylibHandle};
use viktoe_chess::{board::{GameState, Turn}, piece::Color, prelude::BoardPosition, ChessError};

use crate::scenes::{game::screen::screen_to_board_coord, SceneStorage};

pub fn update_mouse_action(rl: &mut RaylibHandle, scene: &mut SceneStorage) {
    scene.game.mouse = rl.get_mouse_position();
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        mouse_left_click(rl, scene);
    }
}

fn mouse_left_click(rl: &mut RaylibHandle, scene: &mut SceneStorage) {
    let (x, y) = screen_to_board_coord(rl.get_mouse_x(), rl.get_mouse_y());

    if !(0..8).contains(&x) || !(0..8).contains(&y) {
        return;
    }

    // check if clicked index was already selected
    if let Some(selected_index) = scene.game.selected_slot {
        // if selected_index == clicked_index
        if selected_index != y * 8 + x {
            let (sx, sy) = ((selected_index % 8) as u8, (selected_index / 8) as u8);
            let moved = scene.chess.move_piece(
                &BoardPosition::try_from((sx, sy)).unwrap(), 
                &BoardPosition::try_from((x as u8, y as u8)).unwrap());
            
            if moved.is_ok() {
                let turn = match scene.chess.get_player_turn() {
                    Turn::White => Turn::White,
                    Turn::Black => Turn::Black,
                };
                match moved.unwrap() {
                    GameState::Promotion(..) => { scene.game.promoted = (turn, ((sx + (sy & 1) + 1) & 1) as usize) }
                    _ => {}
                }

            }

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
                // selected_slot = Some(clicked_index)
                scene.game.selected_slot = Some(y * 8 + x);
            }
        }
    }
}
