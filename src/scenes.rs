use std::net::TcpStream;

use end::End;
use raylib::prelude::*;
use game::{player::PlayerTypes, Game};
use remoteconn::RemoteConn;
use remotegame::RemoteGame;
use start::Start;

pub mod start;
mod game;
mod end;
pub mod remoteconn;
mod remotegame;

pub enum SceneType {
    None,
    Start(Start),
    RemoteConn(RemoteConn),
    RemoteGame(RemoteGame),
    Game(Game),
    End(End)
}

pub enum SceneInitType {
    None,
    Start,
    RemoteConn,
    RemoteGame(TcpStream),
    Game([PlayerTypes; 2]),
    End
}

pub struct SceneStorage {
    scene: SceneType,
}

impl SceneStorage {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneStorage {
        SceneStorage {
            scene: SceneType::None,
        }
    }
    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        match self.scene {
            SceneType::Start(ref mut start) => start.draw(draw_handler),
            SceneType::RemoteConn(ref mut rconn) => rconn.draw(draw_handler),
            SceneType::RemoteGame(ref mut rgame) => rgame.draw(draw_handler),
            SceneType::Game(ref mut game) => game.draw(draw_handler),
            SceneType::End(ref mut end) => end.draw(draw_handler),
            SceneType::None => {},
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) -> SceneInitType {
        match self.scene {
            SceneType::Start(ref mut start) => start.update(rl),
            SceneType::RemoteConn(ref mut rconn) => rconn.update(rl),
            SceneType::RemoteGame(ref mut rgame) => rgame.update(rl),
            SceneType::Game(ref mut game) => game.update(rl),
            SceneType::End(ref mut end) => end.update(rl),
            SceneType::None => { SceneInitType::None },
        }
    }

    pub fn set_scene(&mut self, scene: SceneInitType, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.scene = match scene {
            SceneInitType::Start => SceneType::Start(Start::init(rl)),
            SceneInitType::RemoteConn => SceneType::RemoteConn(RemoteConn::init(rl, thread)),
            SceneInitType::RemoteGame(stream) => SceneType::RemoteGame(RemoteGame::init(rl, thread, stream)),
            SceneInitType::Game(players) => SceneType::Game(Game::init(rl, thread, players)),
            SceneInitType::End => SceneType::End(End::init(rl, thread)),
            SceneInitType::None => { SceneType::None },
        };
    }
}

pub trait Scene {
    // fn init(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self;
    fn update(&mut self, rl: &mut RaylibHandle) -> SceneInitType;
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle);
}


