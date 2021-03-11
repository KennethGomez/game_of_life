use ggez::event;

use game_of_life::game::state::GameState;
use ggez::conf::{WindowMode, WindowSetup};

pub fn main() -> ggez::GameResult {
    let w_mode: WindowMode = WindowMode::default().dimensions(512f32, 512f32);
    let w_setup: WindowSetup = WindowSetup::default().title("Game of life");

    let cb = ggez::ContextBuilder::new("game_of_life", "Kenneth <kennethgomezfdz@gmail.com>")
        .window_mode(w_mode)
        .window_setup(w_setup);

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new(1)?;
    event::run(ctx, event_loop, state)
}
