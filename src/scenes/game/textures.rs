use raylib::prelude::*;

use viktoe_chess::piece::Piece;
use viktoe_chess::piece::Color;
use viktoe_chess::position::BoardPosition;

use crate::scenes::SceneStorage;

pub const PIECE_COUNT : usize = 6;
pub const PIECE_NAMES : [&'static str ; PIECE_COUNT] = ["pawn", "bishop", "knight", "rook", "queen", "king"];

pub fn piece_to_texture<'a>(textures: &'a [Texture2D ; PIECE_COUNT], piece: &Piece) -> &'a Texture2D {
     match piece {
        Piece::Pawn { .. } =>   &textures[0],
        Piece::Bishop =>        &textures[1],
        Piece::Knight =>        &textures[2],
        Piece::Rook =>          &textures[3],
        Piece::Queen =>         &textures[4],
        Piece::King { .. } =>   &textures[5],
    }
}

pub fn get_texture_for_square(scene: &SceneStorage, x: u8, y: u8) -> Option<&Texture2D> {
    if let Some(colored_piece) = scene.chess.get_square(&BoardPosition::try_from((x, y)).unwrap()) {
        let (piece, textures) = match colored_piece {
            Color::White(piece) => (piece, &scene.game.white_textures),
            Color::Black(piece) => (piece, &scene.game.black_textures),
        };

        Some(piece_to_texture(textures, piece))
    } else {
        None
    }
}


pub fn load_piece_texture(rl: &mut RaylibHandle, thread: &RaylibThread, color: &'static str, name: &'static str) -> Texture2D {
    rl.load_texture(thread, &("textures/".to_string() + color + "_" + name + ".png")).unwrap()
}
