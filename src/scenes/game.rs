use raylib::prelude::*;
use raylib::color::{Color as RayColor};
use viktoe_chess::board::{Board, GameState, MoveType, Turn};

use super::{SceneStorage, SceneType};

mod screen;

mod textures;
use textures::*;

mod mouse_handler;
use mouse_handler::*;

mod components;
mod promotion;

const WHITE_SLOT_COLOR : u32 = 0xedd6b0ff;
const BLACK_SLOT_COLOR : u32 = 0xb88762ff;
const SELECTED_SLOT_COLOR : u32 = 0xdbc34aff;
const ATTACKABLE_SLOT_COLOR : u32 = 0x05050540;

pub struct Game {
    white_textures: [Texture2D ; PIECE_COUNT],
    black_textures: [Texture2D ; PIECE_COUNT],
    colors: [RayColor ; 4],
    selected_slot: Option<i32>,
    selected_piece_moves: Option<Board<MoveType>>,
    mouse: Vector2,
    promoted: (Turn, usize),
}

pub fn draw_scene(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    components::draw_board_background(draw_handler, &scene.game);
    components::draw_attackable_slots(draw_handler, scene);
    components::draw_pieces_on_board(draw_handler, scene);
    components::draw_special_state(draw_handler, scene);
    draw_handler.draw_fps(10, 10);
}

pub fn update(rl: &mut RaylibHandle, scene: &mut SceneStorage) -> SceneType {
    match scene.chess.get_game_state() {
        GameState::Draw | GameState::CheckMate => { return SceneType::End },
        GameState::Promotion(..) => { promotion::update(rl, scene) } // block update_mouse_action and call promotion update
        _ => { update_mouse_action(rl, scene) },
    }

    SceneType::Game
}

impl Game {
    pub fn init(rl: &mut RaylibHandle, thread: &RaylibThread) -> Game {
        Game {
            white_textures: PIECE_NAMES.map(|name| load_piece_texture(rl, thread, "white", name)),
            black_textures: PIECE_NAMES.map(|name| load_piece_texture(rl, thread, "black", name)),
            // [white, black, selected, attackable]
            colors: [ RayColor::get_color(WHITE_SLOT_COLOR), RayColor::get_color(BLACK_SLOT_COLOR), RayColor::get_color(SELECTED_SLOT_COLOR), RayColor::get_color(ATTACKABLE_SLOT_COLOR)],
            mouse: Vector2 { x: 0.0, y: 0.0 },
            selected_slot: None,
            selected_piece_moves: None,
            promoted: (Turn::White, 0),
        }
    }

}
