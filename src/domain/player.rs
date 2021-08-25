use bracket_lib::prelude::*;

use crate::domain::*;

pub struct Player {
    x: i32, // screen x position
    y: i32, // screen y position
    top_height: i32,
    bottom_height: i32,
    wx: i32, // world x position
    velocity: f32,
    physics_wait: f32,
    physics_wait_max: f32,
    terminal_velocity: f32,
    pub is_dead: bool,
}

impl Player {
    pub fn new(x: i32, y: i32, top_height: i32, bottom_height: i32) -> Self {
        Self {
            x,
            y,
            top_height,
            bottom_height,
            wx: 1,
            velocity: 0.0,
            physics_wait: 0.0,
            physics_wait_max: 40.0,
            terminal_velocity: 2.0,
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
        if self.y > self.bottom_height {
            self.is_dead = true;
        }
    }

    pub fn render_info(&self) -> RenderInfo {
        (self.x, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < self.terminal_velocity {
            self.velocity += GRAVITY;
        }
        self.wx += 1; // add horizontal progress in the world space
        self.y += self.velocity as i32;
        if self.y <= self.top_height {
            self.y = self.top_height;
        }
    }

    fn flap_wings(&mut self) {
        self.velocity = -self.terminal_velocity;
    }
}
