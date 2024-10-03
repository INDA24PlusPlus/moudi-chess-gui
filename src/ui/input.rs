use std::sync::atomic::AtomicUsize;
use raylib::prelude::*;
use raylib::color::Color as RayColor;

use crate::{HEIGHT, WIDTH};

use super::UIElementTrait;

pub struct Input {
    id: usize,
    enabled: bool,

    left: i32,
    right: i32,
    top: i32,
    bottom: i32,

    text: String,
    font: WeakFont,
    text_font_size: i32,
    text_width: i32,
    text_height: i32,
    text_x: i32,
    text_y: i32,

    is_selected: bool,

    bg_color: RayColor,
    selected_bg_color: RayColor,
    border_color: RayColor,
}

impl UIElementTrait for Input {
    fn update(&mut self, rl: &mut RaylibHandle) -> bool {
        if !self.enabled {
            return false;
        }

        if rl.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
            self.is_selected = (self.left..self.right).contains(&rl.get_mouse_x()) && (self.top..self.bottom).contains(&rl.get_mouse_y());
        }

        if !self.is_selected {
            return false;
        }

        while let Some(c) = rl.get_char_pressed() {
            self.text.push(c);

            if !self.text_fits() {
                self.text.pop();
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            self.text.pop();
        }
        self.recalc_text();

        rl.is_key_pressed(KeyboardKey::KEY_ENTER)
    }

    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        if !self.enabled {
            return;
        }

        let width = self.right - self.left;
        let height = self.bottom - self.top;
        draw_handler.draw_rectangle(self.left, self.top, width, height, if self.is_selected { self.selected_bg_color} else {self.bg_color});
        draw_handler.draw_rectangle_lines(self.left, self.top, width, height, self.border_color);
        draw_handler.draw_text(self.text.as_str(), self.text_x, self.text_y, self.text_font_size, RayColor::WHITE);
    }

    fn get_id(&self) -> usize { self.id }
}

impl Input {
    pub fn init(font: WeakFont, text_font_size: i32) -> Self {
        static NEXT_ID : AtomicUsize = AtomicUsize::new(0);
        Self {
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            enabled: true,
            left: 0, right: 0, top: 0, bottom: 0,
            text: String::new(), font, text_font_size, text_height: 0, text_width: 0, text_y: 0, text_x: 0,
            is_selected: false,
            bg_color: RayColor::default(),
            selected_bg_color: RayColor::default(),
            border_color: RayColor::default(),
        }
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn set_left(&mut self, value: i32) {
        self.left = value;
    }

    pub fn set_right(&mut self, value: i32) {
        self.right = value;
    }

    pub fn set_top(&mut self, value: i32) {
        self.top = value;
    }

    pub fn set_bottom(&mut self, value: i32) {
        self.bottom = value;
    }

    pub fn set_width(&mut self, width: i32) {
        if self.left != 0 {
            self.right = self.left + width;
        } else if self.right != 0 {
            self.left = self.right - width;
        } else {
            panic!("Must set left or right of button to be able to determine constraints based on width");
        }
    }

    pub fn set_height(&mut self, height: i32) {
        if self.top != 0 {
            self.bottom = self.top + height;
        } else if self.bottom != 0 {
            self.top = self.bottom - height;
        } else {
            panic!("Must set top or bottom of button to be able to determine constraints based on height");
        }
    }

    pub fn center_horizontal_width(&mut self, width: i32) {
        self.left = (WIDTH - width) / 2;
        self.right = (WIDTH + width) / 2;
    }

    pub fn center_vertical_height(&mut self, height: i32) {
        self.top = (HEIGHT - height) / 2;
        self.bottom = (HEIGHT + height) / 2;
    }

    pub fn set_bg_color(&mut self, color: RayColor) {
        self.bg_color = color;
    }

    pub fn set_selected_bg_color(&mut self, color: RayColor) {
        self.selected_bg_color = color;
    }

    pub fn set_border_color(&mut self, color: RayColor) {
        self.border_color = color;
    }

    pub fn recalc_text(&mut self) { 
        let width = self.right - self.left;
        let height = self.bottom - self.top;
        let text_sizes = self.measure_text();

        assert!(self.text_width < width);
        assert!(self.text_height < height);

        self.text_x = self.left + (width - text_sizes.0) / 2;
        self.text_y = self.top + (height - text_sizes.1) / 2;
    }

    fn measure_text(&self) -> (i32, i32) {
        let measure = self.font.measure_text(self.text.as_str(), self.text_font_size as f32, 3.5);
        (measure.x as i32, measure.y as i32)
    }

    fn text_fits(&self) -> bool {
        self.text_width <= self.measure_text().0
    }
}
