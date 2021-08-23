#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const STATUS_BAR_HEIGHT: i32 = 2;
const PHYSICS_WAIT_TIME: f32 = 40.0;
const GRAVITY: f32 = 0.2;
const TERMINAL_VELOCITY: f32 = 2.0;

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::MainMenu,
            player: Player::new(5, 25),
            frame_time: 0.0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Flappy Dragon");
        ctx.print_centered(8, "(P) Start Playing");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > PHYSICS_WAIT_TIME {
            self.frame_time = 0.0;

            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::GameOver;
        }

        ctx.print(1, 1, "Press SPACE to flap.");
        self.player.render(ctx);
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "YOU ARE DEAD");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::MainMenu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::GameOver => self.game_over(ctx),
        }
    }
}

enum GameMode {
    MainMenu,
    Playing,
    GameOver,
}

struct Player {
    wx: i32, // world x position
    y: i32, // screen y position
    velocity: f32,
}

impl Player {
    fn new(wx: i32, y: i32) -> Self {
        Self {
            wx,
            y,
            velocity: 0.0,
        }
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < TERMINAL_VELOCITY {
            self.velocity += GRAVITY;
        }

        self.y += self.velocity as i32;
        if self.y < STATUS_BAR_HEIGHT {
            self.y = STATUS_BAR_HEIGHT;
        }

        self.wx += 1;
    }

    fn flap(&mut self) {
        self.velocity = -TERMINAL_VELOCITY; // upwards movement
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.set(5, self.y, YELLOW, BLACK, to_cp437('@'));
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(ctx, State::new())
}
