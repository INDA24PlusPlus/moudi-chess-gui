use std::ops::RangeBounds;

use raylib::prelude::*;
use raylib::color::{Color as RayColor};

use viktoe_chess::board::{GameState, Turn};
use viktoe_chess::piece::Piece;

use crate::scenes::SceneStorage;

use super::piece_to_texture;
use super::screen::{board_coord_to_screen, screen_to_board_coord, TILE_SIZE};

const PROMOTION_PIECETYPES : [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop];

pub fn draw(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    let textures = match scene.game.promoted.0 {
        Turn::White => &scene.game.white_textures,
        Turn::Black => &scene.game.black_textures,
    };
    let color = scene.game.colors[scene.game.promoted.1];
    for (i, piece) in PROMOTION_PIECETYPES.iter().enumerate() {
        draw_promotion_square_with_texture(draw_handler, piece_to_texture(textures, piece), color, 9, 5 - i as i32);
    }
}

pub fn update(rl: &mut RaylibHandle, scene: &mut SceneStorage) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let (x, y) = screen_to_board_coord(rl.get_mouse_x(), rl.get_mouse_y());

        if x != 9 || !(2..=5).contains(&y) {
            println!("({}, {})", x, y);
            return;
        }

        match PROMOTION_PIECETYPES[(5 - y) as usize] {
            Piece::Queen => scene.chess.promote_pawn(Piece::Queen),
            Piece::Rook => scene.chess.promote_pawn(Piece::Rook),
            Piece::Knight => scene.chess.promote_pawn(Piece::Knight),
            Piece::Bishop => scene.chess.promote_pawn(Piece::Bishop),
            _ => return,
        }.unwrap();
    }
}

fn draw_promotion_square_with_texture(draw_handler: &mut RaylibDrawHandle, texture: &Texture2D, color: RayColor, x: i32, y: i32) {
    let (px, py) = board_coord_to_screen(x, y);
    draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, color);
    draw_handler.draw_texture(texture, px, py, RayColor::WHITE);
}
