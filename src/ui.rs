use button::Button;
use input::Input;
use label::Label;

use crate::scenes::{Scene, SceneInitType};

pub mod animate;
pub mod button;
pub mod input;
pub mod label;

pub trait UIElementTrait {
    // Return true if element action has been activated
    fn update(&mut self, rl: &mut raylib::RaylibHandle) -> bool;
    fn draw(&self, draw_handler: &mut raylib::prelude::RaylibDrawHandle);
    fn get_id(&self) -> usize;
}

pub enum UIElement {
    Button(Button),
    Input(Input),
    Label(Label)
}

impl UIElementTrait for UIElement {
    fn update(&mut self, rl: &mut raylib::RaylibHandle) -> bool {
        match self {
            UIElement::Button(button) => button.update(rl),
            UIElement::Input(input) => input.update(rl),
            UIElement::Label(label) => label.update(rl),
        }
    }

    fn draw(&self, draw_handler: &mut raylib::prelude::RaylibDrawHandle) {
        match self {
            UIElement::Button(button) => button.draw(draw_handler),
            UIElement::Input(input) => input.draw(draw_handler),
            UIElement::Label(label) => label.draw(draw_handler),
        }
    }

    fn get_id(&self) -> usize {
        match self {
            UIElement::Button(button) => button.get_id(),
            UIElement::Input(input) => input.get_id(),
            UIElement::Label(label) => label.get_id(),
        }
    }
}

