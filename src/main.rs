use scenes::SceneStorage;
use raylib::prelude::*;

mod scenes;

const WIDTH : i32 = 1280;
const HEIGHT : i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Chess")
        .build();

    let mut scene = SceneStorage::new(&mut rl, &thread);

    while !rl.window_should_close() {
        scenes::update(&mut rl, &mut scene);

        let mut draw_handler = rl.begin_drawing(&thread); 
        draw_handler.clear_background(Color::BLACK);
        scenes::draw(&mut draw_handler, &scene);
    }
}
