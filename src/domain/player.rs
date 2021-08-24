use bracket_lib::prelude::*;

use crate::domain::*;

pub struct Player {
    wx: i32, // world x position
    y: i32,  // screen x position
    velocity: f32,
    physics_wait: f32,
    physics_wait_max: f32,
    pub is_dead: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            wx: 5,
            y: 25,
            velocity: 0.0,
            physics_wait: 0.0,
            physics_wait_max: 40.0,
            is_dead: false,
        }
    }

    pub fn accept_key(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(VirtualKeyCode::Space) = key {
            self.flap_wings();
        }
    }

    pub fn update(&mut self, frame_time: f32) {
        self.physics_wait += frame_time;
        if self.physics_wait > self.physics_wait_max {
            self.physics_wait = 0.0;
            self.gravity_and_move();
        }
        if self.y > SCREEN_HEIGHT {
            self.is_dead = true;
        }
    }

    pub fn render_info(&self) -> RenderInfo {
        (5, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < TERMINAL_VELOCITY {
            self.velocity += GRAVITY;
        }
        self.wx += 1; // add horizontal progress in the world space
        self.y += self.velocity as i32;
        if self.y <= STATUS_BAR_HEIGHT {
            self.y = STATUS_BAR_HEIGHT;
        }
    }

    fn flap_wings(&mut self) {
        self.velocity = -TERMINAL_VELOCITY;
    }
}
