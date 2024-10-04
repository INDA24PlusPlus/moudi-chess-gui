use crate::ui::animate::*;
use player::{Player, PlayerTypes};
use raylib::prelude::*;
use raylib::color::Color as RayColor;
use viktoe_chess::{board::{GameState, Turn}, piece::Color, prelude::BoardPosition, ChessGame};

use super::{Scene, SceneInitType, SceneStorage};

use crate::FPS;

mod screen;

mod textures;
use textures::*;

mod components;
mod promotion;
pub mod player;

const WHITE_SLOT_COLOR : u32 = 0xedd6b0ff;
const BLACK_SLOT_COLOR : u32 = 0xb88762ff;
const SELECTED_SLOT_COLOR : u32 = 0xdbc34aff;
const ATTACKABLE_SLOT_COLOR : u32 = 0x05050540;
const TURN_VISUAL_COLOR : u32 = 0xebc334ff;
const KING_SQUARE_IN_CHECK : u32 = 0xf55742ff;

pub struct Game {
    chess: ChessGame,
    white_textures: [Texture2D ; PIECE_COUNT],
    black_textures: [Texture2D ; PIECE_COUNT],
    colors: [RayColor ; 6],
    players: [PlayerTypes; 2],
    player_turn: Turn,
    promoted_slot: Option<i32>,
    king_index: (i32, i32),
    animation: Animation,
}

impl Scene for Game {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        self.draw_board_background(draw_handler);
        self.draw_special_state(draw_handler);
        self.draw_attackable_slots(draw_handler);
        self.draw_pieces_on_board(draw_handler);
        self.draw_player_turn_bar(draw_handler);
    }

    fn update(&mut self, rl: &mut RaylibHandle, _: &RaylibThread) -> SceneInitType {
        match self.chess.get_game_state() {
            GameState::Ongoing => self.on_ongoing(rl),
            GameState::Promotion(..) => {
                if self.get_player_mut().on_promotion() {
                    self.update_promotion(rl);
                }
            },
            GameState::CheckMate | GameState::Draw => self.get_player_mut().on_end(),
            _ => {},
        }

        SceneInitType::None
    }
}

impl Game {
    pub fn init(rl: &mut RaylibHandle, thread: &RaylibThread, players: [PlayerTypes; 2]) -> Self {
        Game {
            chess: ChessGame::default(),
            white_textures: PIECE_NAMES.map(|name| load_piece_texture(rl, thread, "white", name)),
            black_textures: PIECE_NAMES.map(|name| load_piece_texture(rl, thread, "black", name)),
            // [white, black, selected, attackable]
            colors: [ RayColor::get_color(WHITE_SLOT_COLOR), RayColor::get_color(BLACK_SLOT_COLOR), RayColor::get_color(SELECTED_SLOT_COLOR), RayColor::get_color(ATTACKABLE_SLOT_COLOR), RayColor::get_color(TURN_VISUAL_COLOR), RayColor::get_color(KING_SQUARE_IN_CHECK)],
            players,
            player_turn: Turn::White,
            promoted_slot: None,
            king_index: (4, 7 * 8 + 4),
            animation: Animation::new(Animations::EaseInOutCirc, (0.2 * FPS as f32) as u32),
        }
    }

    fn on_ongoing(&mut self, rl: &mut RaylibHandle) {
        self.get_player_mut().on_ongoing(rl);

        if let Some(selected) = self.get_player().get_selected_slot() {
            let (sx, sy) = (selected as u8 % 8, selected as u8 / 8);
            let from = BoardPosition::try_from((sx, sy)).unwrap();

            if let Some(move_to) = self.get_player().get_move() {
                let to = BoardPosition::try_from((move_to as u8 % 8, move_to as u8 / 8)).unwrap();

                if !self.get_player_mut().on_move_piece() {
                    return;
                }

                let moved =  self.chess.move_piece(&from, &to);

                if moved.is_ok() {
                    self.player_turn = match self.chess.get_player_turn() {
                        Turn::White => Turn::White,
                        Turn::Black => Turn::Black,
                    };

                    self.animation.restart();
                    self.update_king_index(selected, move_to);

                    if let GameState::Promotion(..) = moved.unwrap() {
                        self.promoted_slot = Some(((sx + (sy & 1) + 1) & 1) as i32)
                    }
                }
            } else if let Some(piece) = self.chess.get_square(&from) {
                let turn = self.chess.get_player_turn();

                if !((matches!(piece, Color::White(_)) && matches!(turn, Turn::White)) || (matches!(piece, Color::Black(_)) && matches!(turn, Turn::Black))) {
                    self.get_player_mut().clear_selected();
                }
            } else {
                self.get_player_mut().clear_selected();
            }
        }
    }

    fn update_king_index(&mut self, from: i32, to: i32) {
        // inverse since the turn has shifted to the opponent player since the move was made
        match self.chess.get_player_turn() {
            Turn::Black => {
                if self.king_index.0 == from {
                    self.king_index.0 = to;
                }
            },
            Turn::White => {
                if self.king_index.1 == from {
                    self.king_index.1 = to;
                }
            }
        }
    }

    pub fn get_player(&self) -> &impl Player {
        match self.player_turn {
            Turn::White => &self.players[0],
            Turn::Black => &self.players[1],
        }
    }

    pub fn get_player_mut(&mut self) -> &mut impl Player {
        match self.player_turn {
            Turn::White => &mut self.players[0],
            Turn::Black => &mut self.players[1],
        }
    }

    pub fn get_game_state(&self) -> &GameState {
        self.chess.get_game_state()
    }

    pub fn get_player_turn(&self) -> &Turn {
        self.chess.get_player_turn()
    }
}
