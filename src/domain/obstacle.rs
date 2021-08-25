use bracket_lib::prelude::*;
use std::cmp;

use crate::domain::*;

#[derive(Debug)]
pub struct Obstacle {
    x: i32,
    gap_y: i32,
    gap_size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut rand_gen = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: rand_gen.range(PLAY_ZONE_TOP_Y, SCREEN_HEIGHT),
            gap_size: cmp::max(2, 20 - score),
        }
    }

    pub fn update(&mut self) {
        self.x -= 1;
    }

    pub fn render_info(&self) -> Vec<RenderInfo> {
        let mut render_info = vec![];
        for y in PLAY_ZONE_TOP_Y..SCREEN_HEIGHT {
            if y < self.gap_y - (self.gap_size / 2) || y > self.gap_y + (self.gap_size / 2) {
                render_info.push((self.x, y, RED, WHITE, to_cp437('|')));
            }
        }
        render_info
    }

    pub fn hits(&self, x: i32, y: i32) -> bool {
        let x_match = x == self.x;
        let y_match = y < self.gap_y - (self.gap_size / 2) || y > self.gap_y + (self.gap_size / 2);
        x_match && y_match
    }

    pub fn passes(&self, x: i32, y: i32) -> bool {
        let x_match = x == self.x;
        let y_match =
            y >= self.gap_y - (self.gap_size / 2) && y <= self.gap_y + (self.gap_size / 2);
        x_match && y_match
    }

    pub fn should_dispose(&self) -> bool {
        self.x < 0
    }
}
