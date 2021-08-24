use bracket_lib::prelude::VirtualKeyCode;

pub struct GameOverState {
    pub play_again: bool,
    pub quit_game: bool,
}

impl GameOverState {
    pub fn new() -> Self {
        Self {
            play_again: false,
            quit_game: false,
        }
    }

    pub fn accept_key(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(key) = key {
            match key {
                VirtualKeyCode::P => self.play_again = true,
                VirtualKeyCode::Q => self.quit_game = true,
                _ => {}
            }
        }
    }

    pub fn reset(&mut self) {
        self.play_again = false;
        self.quit_game = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_p_key_play_again() {
        let mut state = GameOverState::new();

        state.accept_key(Some(VirtualKeyCode::P));

        assert!(state.play_again);
        assert!(!state.quit_game);
    }

    #[test]
    fn accept_q_key_quit_game() {
        let mut state = GameOverState::new();

        state.accept_key(Some(VirtualKeyCode::Q));

        assert!(!state.play_again);
        assert!(state.quit_game);
    }

    #[test]
    fn accept_other_keys_does_nothing() {
        let mut state = GameOverState::new();

        state.accept_key(Some(VirtualKeyCode::X));

        assert!(!state.play_again);
        assert!(!state.quit_game);
    }
}
