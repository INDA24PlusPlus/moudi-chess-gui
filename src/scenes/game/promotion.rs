use raylib::prelude::*;
use raylib::color::{Color as RayColor};

use viktoe_chess::board::Turn;
use viktoe_chess::piece::Piece;

use super::{piece_to_texture, Game};
use super::screen::{board_coord_to_screen, screen_to_board_coord, TILE_SIZE};

const PROMOTION_PIECETYPES : [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop];

impl Game {
    pub fn draw_promotion(&self, draw_handler: &mut RaylibDrawHandle) {
        if let Some(promotion_slot) = self.promoted_slot {
            let textures = match self.player_turn {
                Turn::White => &self.white_textures,
                Turn::Black => &self.black_textures,
            };

            let (px, py) = (promotion_slot as usize % 8, promotion_slot as usize / 8);
            let color = self.colors[(px + (py & 1) + 1) & 1];
            for (i, piece) in PROMOTION_PIECETYPES.iter().enumerate() {
                draw_promotion_square_with_texture(draw_handler, piece_to_texture(textures, piece), color, 9, 5 - i as i32);
            }
        }
    }

    pub fn update_promotion(&mut self, rl: &mut RaylibHandle) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let (x, y) = screen_to_board_coord(rl.get_mouse_x(), rl.get_mouse_y());

            if x != 9 || !(2..=5).contains(&y) {
                return;
            }

            match PROMOTION_PIECETYPES[(5 - y) as usize] {
                Piece::Queen => self.chess.promote_pawn(Piece::Queen),
                Piece::Rook => self.chess.promote_pawn(Piece::Rook),
                Piece::Knight => self.chess.promote_pawn(Piece::Knight),
                Piece::Bishop => self.chess.promote_pawn(Piece::Bishop),
                _ => return,
            }.unwrap();
        }
    }
}

fn draw_promotion_square_with_texture(draw_handler: &mut RaylibDrawHandle, texture: &Texture2D, color: RayColor, x: i32, y: i32) {
    let (px, py) = board_coord_to_screen(x, y);
    draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, color);
    draw_handler.draw_texture(texture, px, py, RayColor::WHITE);
}
