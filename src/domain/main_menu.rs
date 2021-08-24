use bracket_lib::prelude::VirtualKeyCode;

pub struct MainMenuState {
    pub start_playing: bool,
    pub quit_game: bool,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self {
            start_playing: false,
            quit_game: false,
        }
    }

    pub fn accept_key(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(key) = key {
            match key {
                VirtualKeyCode::P => self.start_playing = true,
                VirtualKeyCode::Q => self.quit_game = true,
                _ => {}
            }
        }
    }

    pub fn reset(&mut self) {
        self.start_playing = false;
        self.quit_game = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_p_key_start_playing() {
        let mut state = MainMenuState::new();

        state.accept_key(Some(VirtualKeyCode::P));

        assert!(state.start_playing);
        assert!(!state.quit_game);
    }

    #[test]
    fn accept_q_key_quit_game() {
        let mut state = MainMenuState::new();

        state.accept_key(Some(VirtualKeyCode::Q));

        assert!(!state.start_playing);
        assert!(state.quit_game);
    }

    #[test]
    fn accept_other_keys_does_nothing() {
        let mut state = MainMenuState::new();

        state.accept_key(Some(VirtualKeyCode::X));

        assert!(!state.start_playing);
        assert!(!state.quit_game);
    }
}
