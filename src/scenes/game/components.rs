use raylib::prelude::*;
use raylib::color::{Color as RayColor};
use viktoe_chess::board::{GameState, Turn};
use viktoe_chess::prelude::BoardPosition;

use crate::scenes::game::screen::BOARD_OFFSET_X;

use super::player::Player;
use super::{SceneStorage, Game, screen, promotion};

use super::screen::{board_coord_to_screen, ATTACK_CIRCLE_RADIUS, BOARD_OFFSET_Y, BOARD_SIZE, IMAGE_SIZE, TILE_SIZE};

const ATTACK_RING_WIDTH  : f32 = 5.0;
const ATTACK_RING_PADDING : f32 = 3.0;
const ATTACK_INNER_RADIUS : f32 = (TILE_SIZE as f32 / 2.0) - ATTACK_RING_WIDTH - ATTACK_RING_PADDING;
const ATTACK_OUTER_RADIUS : f32 = (TILE_SIZE as f32 / 2.0) - ATTACK_RING_PADDING;

const TURN_VISUAL_OFFSET : i32 = 15;
const TURN_VISUAL_HEIGHT : i32 = 5;
const TURN_VISUAL_WIDTH : i32 = BOARD_SIZE - 20;

impl Game {
    pub fn draw_board_background(&self, draw_handler: &mut RaylibDrawHandle) {
        let mut color_index = 0;
        for y in 0..8 {
            for x in 0..8 {
                let (px, py) = screen::board_coord_to_screen(x, y);
                draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, self.colors[color_index]);
                color_index = (color_index + 1) & 1;
            }
            color_index = (color_index + 1) & 1;
        }
        
        if let Some(index) = self.get_player().get_selected_slot() {
            let (px, py) = screen::board_coord_to_screen(index % 8, index / 8);
            draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, self.colors[2]);
        }
    }

    pub fn draw_pieces_on_board(&self, draw_handler: &mut RaylibDrawHandle) {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(skip) = self.get_player().get_selected_slot() {
                    if y * 8 + x == skip {
                        continue;
                    }
                }
                if let Some(texture) = self.get_texture_for_square(x as u8, y as u8) {
                    let (px, py) = screen::board_coord_to_screen(x, y);
                    draw_handler.draw_texture(texture, px, py, RayColor::WHITE);
                }
            }
        }

        if let Some(index) = self.get_player().get_selected_slot() {
            if let Some(texture) = self.get_texture_for_square((index % 8) as u8, (index / 8) as u8) {
                let mouse = self.get_player().get_mouse();
                draw_handler.draw_texture(texture, mouse.0 as i32 - IMAGE_SIZE / 2, mouse.1 as i32 - IMAGE_SIZE / 2, RayColor::WHITE);
            }
        }
    }

    pub fn draw_attackable_slots(&self, draw_handler: &mut RaylibDrawHandle) {
        if let Some(slot) = self.get_player().get_selected_slot() {
            let moves = self.chess.get_valid_moves(&BoardPosition::try_from((slot as u8 % 8, slot as u8 / 8)).unwrap());

            for y in 0..8u8 {
                for x in 0..8u8 {
                    let pos = BoardPosition::try_from((x, y)).unwrap();
                    if moves.get(&pos).is_none() {
                        continue;
                    }

                    let (px, py) = screen::board_coord_to_screen(x as i32, y as i32);
                    if self.chess.get_square(&pos).is_some() {
                        let center = Vector2{ x: (px + TILE_SIZE / 2) as f32, y: (py + TILE_SIZE / 2) as f32 };
                        draw_handler.draw_ring(center, ATTACK_INNER_RADIUS, ATTACK_OUTER_RADIUS, 0.0, 360.0, 1, self.colors[3]);
                    } else {
                        draw_handler.draw_circle(px + (TILE_SIZE) / 2, py + (TILE_SIZE) / 2, ATTACK_CIRCLE_RADIUS as f32, self.colors[3]);
                    }
                }
            }
        }
    }

    pub fn draw_player_turn_bar(&mut self, draw_handler: &mut RaylibDrawHandle) {
        let py = match self.chess.get_player_turn() {
            Turn::White => BOARD_OFFSET_Y + BOARD_SIZE + TURN_VISUAL_OFFSET,
            Turn::Black => BOARD_OFFSET_Y - TURN_VISUAL_OFFSET - TURN_VISUAL_HEIGHT,
        };
        
        let bar_width = ((TURN_VISUAL_WIDTH as f32 * self.animation.next()).floor() as i32).abs();
        let px = BOARD_OFFSET_X + (BOARD_SIZE - bar_width) / 2;

        draw_handler.draw_rectangle(px, py, bar_width, TURN_VISUAL_HEIGHT, self.colors[4]);
    }

    pub fn draw_special_state(&self, draw_handler: &mut RaylibDrawHandle) {
        match self.chess.get_game_state() {
            GameState::Check => self.draw_king_in_check(draw_handler),
            GameState::Promotion(..) => self.draw_promotion(draw_handler),
            _ => {}
        }
    }

    fn draw_king_in_check(&self, draw_handler: &mut RaylibDrawHandle) {
        let index = match self.chess.get_player_turn() {
            Turn::White => self.king_index.0,
            Turn::Black => self.king_index.1,
        };
        
        let (px, py) = board_coord_to_screen(index % 8, index / 8);
        draw_handler.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, self.colors[5]);
    }

}
