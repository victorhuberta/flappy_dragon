use bracket_lib::prelude::VirtualKeyCode;

use crate::domain::*;
use crate::infra::MyRNG;

const SPAWN_OBSTACLE_WAIT_MAX: f32 = 1500.0;

pub struct PlayState {
    rng: MyRNG,
    player: Player,
    obstacles: Vec<Obstacle>,
    spawn_obstacle_wait: f32,
    score: i32,
    is_game_over: bool,
}

impl PlayState {
    pub fn new(rng: MyRNG) -> Self {
        Self {
            rng,
            player: Player::new(5, 25),
            obstacles: vec![],
            spawn_obstacle_wait: 0.0,
            score: 0,
            is_game_over: false,
        }
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn accept_key(&mut self, key: Option<VirtualKeyCode>) {
        self.player.accept_key(key);
    }

    pub fn update(&mut self, frame_time: f32) {
        self.spawn_obstacle_wait += frame_time;
        if self.spawn_obstacle_wait > SPAWN_OBSTACLE_WAIT_MAX {
            self.spawn_obstacle_wait = 0.0;
            self.obstacles
                .push(Obstacle::new(SCREEN_WIDTH, self.score, &mut self.rng));
        }

        let mut idx = 0;
        while idx < self.obstacles.len() {
            let obstacle = &mut self.obstacles[idx];
            obstacle.update();
            if obstacle.should_dispose() {
                self.obstacles.remove(idx);
                continue;
            }
            if obstacle.hits(self.player.x(), self.player.y()) {
                self.player.is_dead = true;
            }
            if obstacle.passes(self.player.x(), self.player.y()) {
                self.score += 1;
            }
            idx += 1;
        }

        self.player.update(frame_time);
        self.is_game_over = self.player.is_dead;
    }

    pub fn canvas(&self) -> PlayCanvas {
        PlayCanvas {
            player: self.player.render_info(),
            obstacles: self.obstacles.iter().map(Obstacle::render_info).collect(),
        }
    }

    pub fn reset(&mut self) {
        self.player = Player::new(5, 25);
        self.obstacles = vec![];
        self.spawn_obstacle_wait = 0.0;
        self.score = 0;
        self.is_game_over = false;
    }
}

#[derive(Debug, PartialEq)]
pub struct PlayCanvas {
    pub player: RenderInfo,
    pub obstacles: Vec<Vec<RenderInfo>>,
}

#[cfg(test)]
mod tests {
    use bracket_lib::prelude::*;

    use super::*;

    // All updates are performed under 10s.
    const LONG_FRAME_TIME: f32 = 10000.0;

    #[test]
    fn begins_with_correct_state() {
        let state = PlayState::new(MyRNG::new());

        assert_eq!(state.score(), 0);
        assert!(!state.is_game_over());
    }

    #[test]
    fn player_falls_to_their_death() {
        let mut state = PlayState::new(MyRNG::new());

        while !state.is_game_over() {
            state.update(LONG_FRAME_TIME);
        }

        assert!(state.is_game_over());
    }

    #[test]
    fn player_flies_to_topmost_height() {
        let mut state = PlayState::new(MyRNG::new());

        let (_, mut prev_player_y, _, _, _) = state.canvas().player;
        let mut player_y = 0;
        while player_y != prev_player_y {
            state.accept_key(Some(VirtualKeyCode::Space));
            state.update(LONG_FRAME_TIME);

            prev_player_y = player_y;
            let (_, y, _, _, _) = state.canvas().player;
            player_y = y;
        }

        assert_eq!(
            state.canvas().player,
            (5, PLAY_ZONE_TOP_Y, YELLOW, BLACK, to_cp437('@'))
        );
    }

    #[test]
    fn first_obstacle_moves_toward_player() {
        let mut state = PlayState::new(MyRNG::new());
        state.update(LONG_FRAME_TIME); // spawn first obstacle

        let (mut prev_x, _, _, _, _) = state.canvas().obstacles[0][0];
        for _ in 0..10 {
            state.update(LONG_FRAME_TIME);

            let (x, _, _, _, _) = state.canvas().obstacles[0][0];
            assert!(x < prev_x);
            prev_x = x;
        }
    }

    #[test]
    fn player_passes_obstacles() {
        // Arrange
        let mut rng = MyRNG::faux();
        faux::when!(rng.range_i32).then_return(PLAY_ZONE_TOP_Y);

        let mut state = PlayState::new(rng);

        // Act (and assert)
        let mut prev_score = 0;
        while state.score() < 30 {
            // pass at least 30 obstacles
            state.accept_key(Some(VirtualKeyCode::Space));
            state.update(LONG_FRAME_TIME);

            let (player_x, _, _, _, _) = state.canvas().player;
            let (obstacle_x, _, _, _, _) = state.canvas().obstacles[0][0];
            // The score increases for every obstacle passed.
            if player_x == obstacle_x {
                assert!(state.score() > prev_score);
                prev_score = state.score();
            }
        }

        // Assert
        assert_eq!(state.score(), 30);
        assert!(!state.is_game_over());
    }

    #[test]
    fn player_hits_obstacle_then_die() {
        // Arrange
        let mut rng = MyRNG::faux();
        faux::when!(rng.range_i32).then_return(SCREEN_HEIGHT - 1);

        let mut state = PlayState::new(rng);

        // Act
        while !state.is_game_over() {
            state.accept_key(Some(VirtualKeyCode::Space));
            state.update(LONG_FRAME_TIME);
        }

        // Assert
        assert!(state.is_game_over());
        assert_eq!(state.score(), 0);
    }

    #[test]
    fn reset_state() {
        let mut state = PlayState::new(MyRNG::new());
        let ori_canvas = state.canvas();
        // Let player fall to their death.
        while !state.is_game_over() {
            state.update(LONG_FRAME_TIME);
        }

        state.reset();

        assert_eq!(state.canvas(), ori_canvas);
        assert_eq!(state.score(), 0);
        assert!(!state.is_game_over());
    }
}
