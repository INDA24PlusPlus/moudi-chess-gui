use raylib::prelude::*;
use raylib::color::{Color as RayColor};
use viktoe_chess::board::GameState;
use viktoe_chess::prelude::BoardPosition;

use super::{SceneStorage, Game, screen, promotion};

use super::screen::{ATTACK_CIRCLE_RADIUS, IMAGE_SIZE, TILE_SIZE};
use super::textures::get_texture_for_square;

const RING_WIDTH  : f32 = 5.0;
const RING_PADDING : f32 = 3.0;
const INNER_RADIUS : f32 = (TILE_SIZE as f32 / 2.0) - RING_WIDTH - RING_PADDING;
const OUTER_RADIUS : f32 = (TILE_SIZE as f32 / 2.0) - RING_PADDING;

pub fn draw_board_background(draw_handler: &mut RaylibDrawHandle, game: &Game) {
    let mut color_index = 0;
    for y in 0..8 {
        for x in 0..8 {
            let (px, py) = screen::board_coord_to_screen(x, y);
            draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, game.colors[color_index]);
            color_index = (color_index + 1) & 1;
        }
        color_index = (color_index + 1) & 1;
    }
    
    if let Some(index) = game.selected_slot {
        let (px, py) = screen::board_coord_to_screen(index % 8, index / 8);
        draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, game.colors[2]);
    }

}

pub fn draw_pieces_on_board(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    for y in 0..8 {
        for x in 0..8 {
            if let Some(skip) = scene.game.selected_slot {
                if y * 8 + x == skip {
                    continue;
                }
            }
            if let Some(texture) = get_texture_for_square(scene, x as u8, y as u8) {
                let (px, py) = screen::board_coord_to_screen(x, y);
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

pub fn draw_attackable_slots(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    if let Some(moves) = &scene.game.selected_piece_moves {
        for y in 0..8u8{
            for x in 0..8u8 {
                let pos = BoardPosition::try_from((x, y)).unwrap();
                if let Some(movetype) = moves.get(&pos) {
                    let (px, py) = screen::board_coord_to_screen(x as i32, y as i32);
                    if scene.chess.get_square(&pos).is_some() {
                        let center = Vector2{ x: (px + TILE_SIZE / 2) as f32, y: (py + TILE_SIZE / 2) as f32 };
                        draw_handler.draw_ring(center, INNER_RADIUS, OUTER_RADIUS, 0.0, 360.0, 1, scene.game.colors[3]);
                    } else {
                        draw_handler.draw_circle(px + (TILE_SIZE) / 2, py + (TILE_SIZE) / 2, ATTACK_CIRCLE_RADIUS as f32, scene.game.colors[3]);
                    }
                }
            }
        }
    }
}


pub fn draw_special_state(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    match scene.chess.get_game_state() {
        GameState::Check => draw_king_in_check(draw_handler, scene),
        GameState::Promotion(..) => promotion::draw(draw_handler, scene),
        _ => {}
    }
}

fn draw_king_in_check(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {

}
