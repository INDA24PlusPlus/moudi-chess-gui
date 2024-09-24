use raylib::prelude::*;
use raylib::color::{Color as RayColor};
use viktoe_chess::board::{Board, MoveType};
use viktoe_chess::prelude::BoardPosition;

use super::{SceneStorage, SceneType};

mod screen;
use screen::{board_coord_to_screen, ATTACK_CIRCLE_RADIUS, IMAGE_SIZE, TILE_SIZE};

mod textures;
use textures::*;

mod mouse_handler;
use mouse_handler::*;

const WHITE_SLOT_COLOR : u32 = 0xedd6b0ff;
const BLACK_SLOT_COLOR : u32 = 0xb88762ff;
const SELECTED_SLOT_COLOR : u32 = 0xdbc34aff;

pub struct Game {
    white_textures: [Texture2D ; PIECE_COUNT],
    black_textures: [Texture2D ; PIECE_COUNT],
    colors: [RayColor ; 3],
    selected_slot: Option<i32>,
    selected_piece_moves: Option<Board<MoveType>>,
    mouse: Vector2,
    disable_slot: bool,
}

pub fn draw_menu(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    draw_board_background(draw_handler, &scene.game);
    draw_pieces_on_board(draw_handler, scene);
    draw_attackable_slots(draw_handler, scene);
    draw_handler.draw_fps(10, 10);
}

pub fn update(rl: &mut RaylibHandle, scene: &mut SceneStorage) -> SceneType {
    update_mouse_action(rl, scene);
    SceneType::Game
}

fn draw_board_background(draw_handler: &mut RaylibDrawHandle, game: &Game) {
    let mut color_index = 0;
    for y in 0..8 {
        for x in 0..8 {
            let (px, py) = board_coord_to_screen(x, y);
            draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, game.colors[color_index]);
            color_index = (color_index + 1) & 1;
        }
        color_index = (color_index + 1) & 1;
    }
    
    if let Some(index) = game.selected_slot {
        let (px, py) = board_coord_to_screen(index % 8, index / 8);
        draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, game.colors[2]);
    }

}

fn draw_pieces_on_board(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    for y in 0..8 {
        for x in 0..8 {
            if let Some(skip) = scene.game.selected_slot {
                if y * 8 + x == skip {
                    continue;
                }
            }
            if let Some(texture) = get_texture_for_square(scene, x as u8, y as u8) {
                let (px, py) = board_coord_to_screen(x, y);
                draw_handler.draw_texture(texture, px, py, RayColor::WHITE);
            }
        }
    }

    if let Some(index) = scene.game.selected_slot {
        if let Some(texture) = get_texture_for_square(scene, (index % 8) as u8, (index / 8) as u8) {
            draw_handler.draw_texture(texture, scene.game.mouse.x as i32 - IMAGE_SIZE / 2, scene.game.mouse.y as i32 - IMAGE_SIZE / 2, RayColor::WHITE);
        }
    }
}

fn draw_attackable_slots(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    if let Some(moves) = &scene.game.selected_piece_moves {
        for y in 0..8u8{
            for x in 0..8u8 {
                if let Some(movetype) = moves.get(&BoardPosition::try_from((x, y)).unwrap()) {
                    let (px, py) = board_coord_to_screen(x as i32, y as i32);
                    draw_handler.draw_circle(px + (TILE_SIZE) / 2, py + (TILE_SIZE) / 2, ATTACK_CIRCLE_RADIUS as f32, RayColor::RED);
                }
            }
        }
    }
}

impl Game {
    pub fn init(rl: &mut RaylibHandle, thread: &RaylibThread) -> Game {
        Game {
            white_textures: PIECE_NAMES.map(|name| load_piece_texture(rl, thread, "white", name)),
            black_textures: PIECE_NAMES.map(|name| load_piece_texture(rl, thread, "black", name)),
            colors: [ RayColor::get_color(WHITE_SLOT_COLOR), RayColor::get_color(BLACK_SLOT_COLOR), RayColor::get_color(SELECTED_SLOT_COLOR)], // [white, black, selected]
            mouse: Vector2 { x: 0.0, y: 0.0 },
            selected_slot: None,
            selected_piece_moves: None,
            disable_slot: false,
        }
    }

}
