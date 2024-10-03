use std::{collections::HashMap, io::Write, net::TcpStream, os::unix::thread};

use chess_networking::Start;
use raylib::prelude::*;
use raylib::color::Color as RayColor;

use button::Button;

use super::{Scene, SceneInitType};
use crate::{ui::{input::Input, *}, HEIGHT, WIDTH};

const PLAY_BUTTON_HEIGHT : i32 = 75;
const FONT_SIZE : i32 = 45;
const PLAY_BUTTON_BOTTOM_PADDING : i32 = 10;

const BACK_BUTTON_PADDING : i32 = 10;
const BACK_BUTTON_SIZE : i32 = 50;

pub struct RemoteConn {
    elements: [UIElement; 3],
    actions: HashMap<usize, Box<dyn Fn(&Self) -> SceneInitType>>
}

impl Scene for RemoteConn {
    fn draw(&mut self, draw_handler: &mut raylib::prelude::RaylibDrawHandle) {
        for element in &self.elements {
            element.draw(draw_handler);
        }
    }

    fn update(&mut self, rl: &mut raylib::RaylibHandle) -> SceneInitType {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return SceneInitType::Start;
        }

        for element in &mut self.elements {
            if element.update(rl) {
                let id = element.get_id();
                if let Some(action) = self.actions.get(&id) {
                    return action(self);
                }
            }
        }

        SceneInitType::None
    }
}

impl RemoteConn {
    pub fn init(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let input_bg_color = RayColor::get_color(0xff_ff_ff_0a);
        let input_selected_color = RayColor::get_color(0xff_ff_ff_1a);
        let input_border_color = RayColor::get_color(0xff_ff_ff_a0);

        let mut input = Input::init(rl.get_font_default(), FONT_SIZE);
        let input_id = input.get_id();
        input.center_vertical_height(FONT_SIZE + 20);
        input.center_horizontal_width(5 * WIDTH / 9);
        input.set_bg_color(input_bg_color);
        input.set_selected_bg_color(input_selected_color);
        input.set_border_color(input_border_color);

        let mut play_button = Button::new(rl.get_font_default());
        let play_button_id = play_button.get_id();
        play_button.center_horizontal_width(WIDTH / 2);
        play_button.set_top(HEIGHT - PLAY_BUTTON_HEIGHT - PLAY_BUTTON_BOTTOM_PADDING);
        play_button.set_height(PLAY_BUTTON_HEIGHT);
        play_button.set_text("Connect and play", FONT_SIZE);

        let mut back_button = Button::new(rl.get_font_default());
        let back_button_id = back_button.get_id();
        back_button.set_top(BACK_BUTTON_PADDING);
        back_button.set_left(BACK_BUTTON_PADDING);
        back_button.set_width(BACK_BUTTON_SIZE);
        back_button.set_height(BACK_BUTTON_SIZE);
        back_button.set_text("<", FONT_SIZE);

        let mut remote_conn = RemoteConn {
            elements: [UIElement::Button(play_button), UIElement::Button(back_button), UIElement::Input(input)],
            actions: HashMap::new()
        };

        let conn = |scene: &Self| {
            if let UIElement::Input(input) = &scene.elements[2] {
                connect(input)
            } else {
                SceneInitType::None
            }
        };

        remote_conn.add_action(play_button_id, Box::new(conn));
        remote_conn.add_action(input_id, Box::new(conn));

        remote_conn.add_action(back_button_id, Box::new(|_: &Self| SceneInitType::Start));

        remote_conn
    }

    fn add_action(&mut self, id: usize, action: Box<dyn Fn(&Self) -> SceneInitType>) {
        self.actions.insert(id, action);
    }
}

pub fn connect(input: &Input) -> SceneInitType {
    let address = input.get_text();
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            let start = Start {
                is_white: true,
                name: Some("Zimon".to_string()),
                fen: None,
                time: None,
                inc: None,
            };

            let buf : Vec<u8> = start.try_into().unwrap();
            if let Err(e) = stream.write_all(&buf) {
                println!("\nError sending start to server: {}\n", e);
                SceneInitType::None
            } else {
                SceneInitType::RemoteGame(stream)
            }
        },
        Err(err) => {
            println!("Error occured trying to connect to: {}", address);
            println!("{}", err);
            SceneInitType::None
        }
    }
}
