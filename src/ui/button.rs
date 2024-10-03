use std::sync::atomic::AtomicUsize;

use ffi::MeasureText;
use raylib::prelude::*;
use raylib::color::Color as RayColor;

use crate::{scenes::{SceneInitType, SceneStorage}, HEIGHT, WIDTH};

use super::{UIElement, UIElementTrait};

pub struct Button
{
    id: usize,
    enabled: bool,

    left: i32,
    right: i32,
    top: i32,
    bottom: i32,

    text: &'static str,
    font: WeakFont,
    text_font_size: i32,
    text_width: i32,
    text_height: i32,
    text_x: i32,
    text_y: i32,

    is_hovering: bool,
    normal_color: RayColor,
    hovered_color: RayColor,
}

impl UIElementTrait for Button {
    fn update(&mut self, rl: &mut RaylibHandle) -> bool {
        if !self.enabled {
            return false;
        }

        self.is_hovering = (self.left..self.right).contains(&rl.get_mouse_x()) && (self.top..self.bottom).contains(&rl.get_mouse_y());
        return self.is_hovering && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    }

    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        if !self.enabled {
            return;
        }

        draw_handler.draw_rectangle(self.left, self.top, self.right - self.left, self.bottom - self.top, 
            if self.is_hovering { self.hovered_color }
            else { self.normal_color }
        );

        draw_handler.draw_text(self.text, self.text_x, self.text_y, self.text_font_size, RayColor::BLACK);
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl Button {
    pub fn new(font: WeakFont) -> Self {
        static NEXT_ID : AtomicUsize = AtomicUsize::new(0);
        Self {
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            enabled: true,
            left: 0, right: 0, top: 0, bottom: 0,
            text: "", font, text_font_size: 0, text_width: 0, text_height: 0, text_x: 0, text_y: 0,
            is_hovering: false,
            normal_color: RayColor::get_color(0xff_ff_ff_50),
            hovered_color: RayColor::get_color(0xff_ff_ff_70),
        }
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
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
        self.bottom = (HEIGHT - height) / 2;
    }

    pub fn set_text(&mut self, text: &'static str, font_size: i32) {
        self.text = text;
        self.text_font_size = font_size;
        
        let width = self.right - self.left;
        let height = self.bottom - self.top;
        // Some made up spacing that seems to center the text correctly?
        let text_size = self.font.measure_text(text, font_size as f32, 3.5);

        self.text_width = text_size.x as i32;
        self.text_height = text_size.y as i32;
 
        assert!(self.text_width < width);
        assert!(self.text_height < height);

        self.text_x = self.left + (width - self.text_width) / 2;
        self.text_y = self.top + (height - self.text_height) / 2;
    }

    pub fn set_color(&mut self, color: RayColor) {
        self.normal_color = color;
    }

    pub fn set_hovered_color(&mut self, color: RayColor) {
        self.hovered_color = color;
    }
}
