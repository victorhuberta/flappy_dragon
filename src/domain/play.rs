use bracket_lib::prelude::VirtualKeyCode;

use crate::domain::*;

pub struct PlayState {
    player: Player,
    pub is_game_over: bool,
}

impl PlayState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            is_game_over: false,
        }
    }

    pub fn accept_key(&mut self, key: Option<VirtualKeyCode>) {
        self.player.accept_key(key);
    }

    pub fn update(&mut self, frame_time: f32) {
        self.player.update(frame_time);
        self.is_game_over = self.player.is_dead;
    }

    pub fn render_info(&self) -> [RenderInfo; 1] {
        [self.player.render_info()]
    }

    pub fn reset(&mut self) {
        self.player = Player::new();
        self.is_game_over = false;
    }
}
