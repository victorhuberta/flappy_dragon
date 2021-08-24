use bracket_lib::prelude::VirtualKeyCode;

use crate::domain::{Player, RenderInfo};

pub struct PlayState {
    player: Player,
    is_game_over: bool,
}

impl PlayState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            is_game_over: false,
        }
    }

    pub fn should_game_over(&self) -> bool {
        self.is_game_over
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_falls_to_their_death() {
        let mut state = PlayState::new();

        // Trigger player's physics 20 times.
        for _ in 0..20 {
            state.update(40.1);
        }

        assert!(state.should_game_over());
    }

    #[test]
    fn reset_state() {
        let mut state = PlayState::new();
        let ori_render_info = state.render_info();
        // Let player fall to their death.
        for _ in 0..20 {
            state.update(40.1);
        }

        state.reset();

        assert_eq!(state.render_info(), ori_render_info);
        assert!(!state.should_game_over());
    }
}
