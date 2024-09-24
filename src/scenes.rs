use raylib::prelude::*;
use game::Game;
use viktoe_chess::board::ChessGame;

mod start;
mod game;
mod end;

pub enum SceneType {
    Start,
    Game,
    End
}

const START_SCENE : SceneType = SceneType::Game;

pub struct SceneStorage {
    scene: SceneType,
    game: Game,
    chess: ChessGame,
}

impl SceneStorage {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneStorage {
        SceneStorage {
            scene: START_SCENE,
            game: Game::init(rl, thread),
            chess: ChessGame::default(),
        }
    }
}

pub fn draw(draw_handler: &mut RaylibDrawHandle, scene: &SceneStorage) {
    match scene.scene {
        SceneType::Start => start::draw_menu(draw_handler),
        SceneType::Game => game::draw_menu(draw_handler, scene),
        SceneType::End => end::draw_menu(draw_handler),
    }
}

pub fn update(rl: &mut RaylibHandle, scene: &mut SceneStorage) -> SceneType {
    match scene.scene {
        SceneType::Start => start::update(),
        SceneType::Game => game::update(rl, scene),
        SceneType::End => end::update(),
    }
}
