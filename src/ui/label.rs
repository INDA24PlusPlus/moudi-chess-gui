use std::{ops::RangeBounds, sync::atomic::AtomicUsize, time::{Duration, SystemTime}};

use raylib::prelude::*;
use raylib::color::Color as RayColor;

use crate::{HEIGHT, WIDTH};

use super::UIElementTrait;

pub struct Label {
    id: usize,
    enabled: bool,

    texts: Vec<&'static str>,
    text_index: usize,
    font: WeakFont,
    font_size: i32,
    color: RayColor,
    width: i32,
    height: i32,
    x: i32,
    y: i32,

    timeout: Duration,
    last_update: SystemTime,
}

impl UIElementTrait for Label {
    fn update(&mut self, rl: &mut raylib::RaylibHandle) -> bool {
        if let Ok(duration) = self.last_update.elapsed() {
            if duration > self.timeout {
                self.last_update += duration;
                let (old_width, old_height) = (self.width, self.height);
                self.next_text();
                self.readjust(old_width, old_height);
            } 
        }
        false
    }

    fn draw(&self, draw_handler: &mut raylib::prelude::RaylibDrawHandle) {
        draw_handler.draw_text(self.texts[self.text_index], self.x, self.y, self.font_size, self.color);
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl Label {
    pub fn new(font: WeakFont, font_size: i32) -> Self {
        static NEXT_ID : AtomicUsize = AtomicUsize::new(0);
        Self {
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            enabled: true,
            texts: vec![],
            text_index: 0,
            font,
            font_size,
            color: RayColor::default(),
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            timeout: Duration::default(),
            last_update: SystemTime::now(),
        }
    }

    pub fn set_index(&mut self, index: usize) {
        assert!((0..self.texts.len()).contains(&index));
        self.text_index = index;
        let size = self.font.measure_text(self.texts[self.text_index], self.font_size as f32, 3.5);
        (self.width, self.height) = (size.x as i32, size.y as i32);
    }

    pub fn add_text(&mut self, text: &'static str) {
        self.texts.push(text);
    }

    pub fn next_text(&mut self) {
        self.text_index = (self.text_index + 1) % self.texts.len();
        let size = self.font.measure_text(self.texts[self.text_index], self.font_size as f32, 3.5);
        (self.width, self.height) = (size.x as i32, size.y as i32);
    }

    pub fn readjust(&mut self, old_width: i32, old_height: i32) {
        self.x += (old_width - self.width) / 2;
        self.y += (old_height - self.height) / 2;
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn set_color(&mut self, color: RayColor) {
        self.color = color;
    }

    pub fn set_timeout(&mut self, value: Duration) {
        self.timeout = value;
    }

    pub fn center_vertically(&mut self) {
        assert!(self.height != 0);
        self.y = (HEIGHT - self.height) / 2;
    }

    pub fn center_horizontally(&mut self) {
        assert!(self.width != 0);
        self.x = (WIDTH - self.width) / 2;
    }

    pub fn set_x(&mut self, value: i32) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: i32) {
        self.y = value;
    }
}
