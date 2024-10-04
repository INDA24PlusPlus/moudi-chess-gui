use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use button::Button;
use raylib::prelude::*;
use raylib::color::Color as RayColor;

use crate::{ui::{button, UIElement, UIElementTrait}, HEIGHT, WIDTH};

use super::{game::player::{local::LocalPlayer, PlayerTypes}, Scene, SceneInitType};

const BUTTON_HEIGHT : i32 = 75;
const FONT_SIZE : i32 = 45;

pub struct Start {
    elements: [UIElement; 2],
    actions: HashMap<usize, Box<dyn Fn(&Self) -> SceneInitType>>
}

impl Scene for Start {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        for element in &self.elements {
            element.draw(draw_handler);
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneInitType {
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

impl Start {
    pub fn init(rl: &mut RaylibHandle) -> Start {
        let normal_color = RayColor::get_color(0xff_ff_ff_50);
        let hovered_color = RayColor::get_color(0xff_ff_ff_70);

        let mut local_play_button = Button::new(rl.get_font_default());
        let local_play_button_id = local_play_button.get_id();
        local_play_button.center_horizontal_width(3 * WIDTH / 9);
        local_play_button.set_top((HEIGHT - (3 * BUTTON_HEIGHT / 2)) / 2);
        local_play_button.set_height(BUTTON_HEIGHT);
        local_play_button.set_text("Local", FONT_SIZE);
        local_play_button.set_color(normal_color);
        local_play_button.set_hovered_color(hovered_color);

        let mut mult_play_button = Button::new(rl.get_font_default());
        let mult_play_button_id = mult_play_button.get_id();
        mult_play_button.center_horizontal_width(3 * WIDTH / 9);
        mult_play_button.set_top((HEIGHT + (3 * BUTTON_HEIGHT / 2)) / 2);
        mult_play_button.set_height(BUTTON_HEIGHT);
        mult_play_button.set_text("Remote", FONT_SIZE);
        mult_play_button.set_color(normal_color);
        mult_play_button.set_hovered_color(hovered_color);

        let mut start = Start {
            elements: [UIElement::Button(local_play_button), UIElement::Button(mult_play_button)],
            actions: HashMap::new()
        };

        start.add_action(local_play_button_id, Box::new(|_: &Self|  SceneInitType::Game([PlayerTypes::Local(LocalPlayer::init()), PlayerTypes::Local(LocalPlayer::init())])));
        start.add_action(mult_play_button_id, Box::new(|_: &Self| SceneInitType::RemoteConn));

        start
    }

    fn add_action(&mut self, id: usize, action: Box<dyn Fn(&Self) -> SceneInitType>) {
        self.actions.insert(id, action);
    }
}
