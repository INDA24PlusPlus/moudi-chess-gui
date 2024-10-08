use std::{collections::HashMap, io::Read, mem, net::TcpStream, time::Duration};

use chess_networking::Start;
use raylib::prelude::*;
use raylib::color::Color as RayColor;

use crate::ui::{button::Button, label::Label, UIElement, UIElementTrait};

use super::{game::{player::{remote_recv::RemoteRecvPlayer, remote_send::RemoteSendPlayer, PlayerTypes}, Game}, Scene, SceneInitType};

const STR_1 : &str = "Waiting for game";
const STR_2 : &str = "Waiting for game.";
const STR_3 : &str = "Waiting for game..";
const STR_4 : &str = "Waiting for game...";

const BACK_BUTTON_PADDING : i32 = 10;
const BACK_BUTTON_SIZE : i32 = 50;

pub struct RemoteGame {
    game: Option<Game>,
    stream: TcpStream,
    elements: [UIElement; 2],
    actions: HashMap<usize, Box<dyn Fn(&Self) -> SceneInitType>>
}

impl Scene for RemoteGame {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        if let Some(game) = &mut self.game {
            game.draw(draw_handler);
        } else {
            for element in &self.elements {
                element.draw(draw_handler);
            }
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneInitType {
        if let Some(game) = &mut self.game {
            game.update(rl, thread);
        } else {
            if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                return SceneInitType::RemoteConn;
            }

            self.update_connection(rl, thread);
            for element in &mut self.elements {
                if element.update(rl) {
                    let id = element.get_id();
                    if let Some(action) = self.actions.get(&id) {
                        return action(self);
                    }
                }
            }
        }

        SceneInitType::None
    }
}

impl RemoteGame {
    pub fn init(rl: &mut RaylibHandle, stream: TcpStream) -> Self {
        // make sure that any stream IO will not be blocking
        stream.set_nonblocking(true).unwrap();

        let mut label = Label::new(rl.get_font_default(), 50);
        label.add_text(STR_1);
        label.add_text(STR_2);
        label.add_text(STR_3);
        label.add_text(STR_4);
        label.set_index(0);
        label.center_vertically();
        label.center_horizontally();
        label.set_color(RayColor::WHITE);
        label.set_timeout(Duration::from_secs(1));

        let mut back_button = Button::new(rl.get_font_default());
        let back_button_id = back_button.get_id();
        back_button.set_top(BACK_BUTTON_PADDING);
        back_button.set_left(BACK_BUTTON_PADDING);
        back_button.set_width(BACK_BUTTON_SIZE);
        back_button.set_height(BACK_BUTTON_SIZE);
        back_button.set_text("<", 45);

        let mut rgame = RemoteGame {
            game: None,
            stream,
            elements: [UIElement::Label(label), UIElement::Button(back_button)],
            actions: HashMap::new()
        };

        rgame.add_action(back_button_id, Box::new(|_: &Self| SceneInitType::RemoteConn));

        rgame
    }

    fn add_action(&mut self, id: usize, action: Box<dyn Fn(&Self) -> SceneInitType>) {
        self.actions.insert(id, action);
    }

    pub fn update_connection(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut buf = [0u8; mem::size_of::<Start>()];
        let _ = self.stream.read(&mut buf);

        if let Ok(start) = chess_networking::Start::try_from(buf.as_slice()) {
            let send_player = PlayerTypes::RemoteSend(RemoteSendPlayer::init(self.stream.try_clone().unwrap()));
            let recv_player = PlayerTypes::RemoteRecv(RemoteRecvPlayer::init(self.stream.try_clone().unwrap()));
            self.game = Some(Game::init(rl, thread, if start.is_white { [send_player, recv_player] } else { [recv_player, send_player] }));
        }
    }
}
