use raylib::prelude::*;

use super::{Scene, SceneInitType, SceneStorage, SceneType};

pub struct End {}

impl Scene for End {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) { }
    fn update(&mut self, rl: &mut raylib::RaylibHandle, thread: &RaylibThread) -> SceneInitType { SceneInitType::None }
}

impl End {
    pub fn init(rl: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread) -> Self {
        Self {}
    }
}

