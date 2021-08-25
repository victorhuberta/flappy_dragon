use bracket_lib::prelude::*;

use crate::domain::*;

const PHYSICS_WAIT_MAX: f32 = 40.0;
const TERMINAL_VELOCITY: f32 = 2.0;

pub struct Player {
    x: i32,
    y: i32,
    velocity: f32,
    physics_wait: f32,
    pub is_dead: bool,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
            physics_wait: 0.0,
            is_dead: false,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn accept_key(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(VirtualKeyCode::Space) = key {
            self.flap_wings();
        }
    }

    pub fn update(&mut self, frame_time: f32) {
        self.physics_wait += frame_time;
        if self.physics_wait > PHYSICS_WAIT_MAX {
            self.physics_wait = 0.0;
            self.gravity_and_move();
        }
        if self.y > SCREEN_HEIGHT {
            self.is_dead = true;
        }
    }

    pub fn render_info(&self) -> RenderInfo {
        (self.x, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < TERMINAL_VELOCITY {
            self.velocity += GRAVITY;
        }
        self.y += self.velocity as i32;
        if self.y <= PLAY_ZONE_TOP_Y {
            self.y = PLAY_ZONE_TOP_Y;
        }
    }

    fn flap_wings(&mut self) {
        self.velocity = -TERMINAL_VELOCITY;
    }
}
