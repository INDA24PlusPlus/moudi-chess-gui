use std::env;

use scenes::{SceneInitType, SceneStorage};
use raylib::prelude::*;
use server::Server;

mod scenes;
mod ui;
mod server;

const WIDTH : i32 = 900;
const HEIGHT : i32 = 700;
pub const FPS : u32 = 60;

fn main() -> std::io::Result<()> {
    
    let args : Vec<String> = env::args().collect();
    let port = match args.len() {
        1 => 5000,
        _ => args[1].parse().unwrap(),
    };

    let (mut server, server_running_signal) = Server::init(port)?;
    let server_thread = std::thread::spawn(move || server.start());

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Chess")
        .build();

    rl.set_target_fps(FPS);
    rl.set_exit_key(Some(KeyboardKey::KEY_DELETE));
    let mut scene = SceneStorage::new(&mut rl, &thread);
    scene.set_scene(SceneInitType::Start, &mut rl, &thread);

    while !rl.window_should_close() {
        let next_scene = scene.update(&mut rl, &thread);

        if !matches!(next_scene, SceneInitType::None) {
            scene.set_scene(next_scene, &mut rl, &thread);
            continue;
        }

        let mut draw_handler = rl.begin_drawing(&thread); 
        draw_handler.clear_background(Color::BLACK);

        scene.draw(&mut draw_handler);
    }

    server_running_signal.store(false, std::sync::atomic::Ordering::SeqCst);
    server_thread.join().unwrap();

    Ok(())
}
