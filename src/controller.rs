use bracket_lib::prelude::*;

use crate::domain::{GameOverState, MainMenuState, PlayState};
use crate::infra::MyRNG;

pub struct State {
    mode: GameMode,
    main_menu: MainMenuState,
    play: PlayState,
    game_over: GameOverState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::MainMenu => self.main_menu_mode(ctx),
            GameMode::Playing => self.play_mode(ctx),
            GameMode::GameOver => self.game_over_mode(ctx),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: GameMode::MainMenu,
            main_menu: MainMenuState::new(),
            play: PlayState::new(MyRNG::new()),
            game_over: GameOverState::new(),
        }
    }

    fn main_menu_mode(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Flappy Dragon");
        ctx.print_centered(8, "(P) Start Playing");
        ctx.print_centered(9, "(Q) Quit Game");

        self.main_menu.accept_key(ctx.key);
        if self.main_menu.start_playing() {
            self.switch_to_play_mode();
        }
        if self.main_menu.quit_game() {
            ctx.quitting = true;
        }
    }

    fn play_mode(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        ctx.print_color(1, 1, BLACK, YELLOW, "Press SPACE to flap wings");
        ctx.print_color(
            1,
            2,
            BLACK,
            YELLOW,
            &format!("Score: {}", self.play.score()),
        );

        self.play.accept_key(ctx.key);
        self.play.update(ctx.frame_time_ms);

        let canvas = self.play.canvas();
        // Render player.
        let (x, y, fg, bg, symbol) = canvas.player;
        ctx.set(x, y, fg, bg, symbol);
        // Render obstacles.
        for obstacle in canvas.obstacles {
            for tile in obstacle {
                let (x, y, fg, bg, symbol) = tile;
                ctx.set(x, y, fg, bg, symbol);
            }
        }

        if self.play.is_game_over() {
            self.switch_to_game_over_mode();
        }
    }

    fn game_over_mode(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, RED, BLACK, "YOU ARE DEAD");
        ctx.print_centered(7, &format!("Score: {}", self.play.score()));
        ctx.print_centered(9, "(P) Play Again");
        ctx.print_centered(10, "(Q) Quit Game");

        self.game_over.accept_key(ctx.key);
        if self.game_over.play_again() {
            self.switch_to_play_mode();
        }
        if self.game_over.quit_game() {
            ctx.quitting = true;
        }
    }

    fn switch_to_play_mode(&mut self) {
        self.play.reset();
        self.mode = GameMode::Playing;
    }

    fn switch_to_game_over_mode(&mut self) {
        self.game_over.reset();
        self.mode = GameMode::GameOver;
    }
}

enum GameMode {
    MainMenu,
    Playing,
    GameOver,
}
