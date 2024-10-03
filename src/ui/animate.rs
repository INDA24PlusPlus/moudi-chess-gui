use std::f32::consts::PI;

pub enum Animations {
    EaseInOutElastic,
    EaseInOutBack,
    EaseOutBounce,
    EaseInOutCirc,
}

pub struct Animation {
    frames_elapsed: u32,
    total_frames: u32,
    animation: Animations,
}

impl Animation {
    pub fn new(animation: Animations, total_frames: u32) -> Animation {
        Animation {
            frames_elapsed: 0,
            total_frames,
            animation
        }
    }

    pub fn next(&mut self) -> f32 {
        let quotient = (self.frames_elapsed as f32) / (self.total_frames as f32);

        if self.frames_elapsed != self.total_frames {
            self.frames_elapsed += 1;
        }

        match self.animation {
            Animations::EaseInOutElastic => ease_in_out_elastic(quotient),
            Animations::EaseInOutBack => ease_in_out_back(quotient),
            Animations::EaseOutBounce => ease_out_bounce(quotient),
            Animations::EaseInOutCirc => ease_in_out_circ(quotient),
        }
    }

    pub fn restart(&mut self) {
        self.frames_elapsed = 0;
    }
}

fn ease_in_out_elastic(x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        -((2.0 as f32).powf(20.0 * x - 10.0) * ((20.0 * x - 11.125) * (2.0 * PI / 4.5)).sin()) / 2.0
    } else {
        ((2.0 as f32).powf(-20.0 * x + 10.0) * ((20.0 * x - 11.125) * (2.0 * PI / 4.5)).sin()) / 2.0 + 1.0
    }
}

fn ease_in_out_back(x: f32) -> f32 {
    const c1 : f32 = 1.70158;
    const c2 : f32 = c1 * 1.525;

    if x < 0.5 {
        ((2.0 * x).powi(2) * ((c2 + 1.0) * 2.0 * x - c2)) / 2.0
    } else {
        ((2.0 * x - 2.0).powi(2) * ((c2 + 1.0) * (x * 2.0 - 2.0) + c2) + 2.0) / 2.0
    }
}

fn ease_out_bounce(x: f32) -> f32 {
    const n : f32 = 7.5625;
    const d : f32 = 2.75;

    if x < 1.0 / d {
        n * x * x
    } else if x < 2.0 / d {
        n * (x - 1.5 / d).powi(2) + 0.75
    } else if x < 2.5 / d {
        n * (x - 2.25 / d).powi(2) + 0.9375
    } else {
        n * (x - 2.625 / d).powi(2) + 0.984375
    }
}

fn ease_in_out_circ(x: f32) -> f32 {
    if x < 0.5 {
        (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}
