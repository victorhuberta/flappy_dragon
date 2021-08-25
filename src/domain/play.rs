use bracket_lib::prelude::VirtualKeyCode;

use crate::domain::*;

pub struct PlayState {
    player: Player,
    is_game_over: bool,
}

impl PlayState {
    pub fn new() -> Self {
        Self {
            player: Player::new(5, 25, STATUS_BAR_HEIGHT, SCREEN_HEIGHT),
            is_game_over: false,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn accept_key(&mut self, key: Option<VirtualKeyCode>) {
        self.player.accept_key(key);
    }

    pub fn update(&mut self, frame_time: f32) {
        self.player.update(frame_time);
        self.is_game_over = self.player.is_dead;
    }

    pub fn canvas(&self) -> PlayCanvas {
        PlayCanvas::new(self.player.render_info())
    }

    pub fn reset(&mut self) {
        self.player = Player::new(5, 25, STATUS_BAR_HEIGHT, SCREEN_HEIGHT);
        self.is_game_over = false;
    }
}

#[derive(Debug, PartialEq)]
pub struct PlayCanvas {
    player: RenderInfo,
}

impl PlayCanvas {
    pub fn new(player: RenderInfo) -> Self {
        Self { player }
    }

    pub fn player(&self) -> RenderInfo {
        self.player
    }
}

#[cfg(test)]
mod tests {
    use bracket_lib::prelude::*;

    use super::*;

    const LONG_FRAME_TIME: f32 = 5000.0; // 5s

    #[test]
    fn player_falls_to_their_death() {
        let mut state = PlayState::new();

        while !state.is_game_over() {
            state.update(LONG_FRAME_TIME);
        }

        assert!(state.is_game_over());
    }

    #[test]
    fn player_flies_to_topmost_height() {
        let mut state = PlayState::new();

        let (_, mut prev_player_y, _, _, _) = state.canvas().player();
        let mut player_y = 0;
        while player_y != prev_player_y {
            state.accept_key(Some(VirtualKeyCode::Space));
            state.update(LONG_FRAME_TIME);

            prev_player_y = player_y;
            let (_, y, _, _, _) = state.canvas().player();
            player_y = y;
        }

        assert_eq!(
            state.canvas().player(),
            (5, STATUS_BAR_HEIGHT, YELLOW, BLACK, to_cp437('@'))
        );
    }

    #[test]
    fn reset_state() {
        let mut state = PlayState::new();
        let ori_canvas = state.canvas();
        // Let player fall to their death.
        while !state.is_game_over() {
            state.update(LONG_FRAME_TIME);
        }

        state.reset();

        assert_eq!(state.canvas(), ori_canvas);
        assert!(!state.is_game_over());
    }
}
