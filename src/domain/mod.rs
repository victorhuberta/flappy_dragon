mod main_menu;
pub use main_menu::*;

mod play;
pub use play::*;

mod game_over;
pub use game_over::*;

mod player;
pub use player::*;

mod obstacle;
pub use obstacle::*;

mod globals {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const PLAY_ZONE_TOP_Y: i32 = 3;
    pub const GRAVITY: f32 = 0.2;

    pub type RenderInfo = (i32, i32, (u8, u8, u8), (u8, u8, u8), u16);
}
pub use globals::*;
