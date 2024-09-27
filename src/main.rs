use scenes::SceneStorage;
use raylib::prelude::*;

mod scenes;

const WIDTH : i32 = 900;
const HEIGHT : i32 = 700;

// TODO:
// 1. Add end scene
// 2. Add start scene with custom fen

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Chess")
        .build();

    let mut scene = SceneStorage::new(&mut rl, &thread);

    while !rl.window_should_close() {
        scene.scene = scenes::update(&mut rl, &mut scene);

        let mut draw_handler = rl.begin_drawing(&thread); 
        draw_handler.clear_background(Color::BLACK);
        scenes::draw(&mut draw_handler, &scene);
    }
}
