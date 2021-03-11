#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ggez::{conf, event};

use game_of_life::game::state::GameState;

pub fn main() -> ggez::GameResult {
    let w_mode: conf::WindowMode = conf::WindowMode::default().dimensions(512.0, 512.0);
    let w_setup: conf::WindowSetup = conf::WindowSetup::default().title("Game of life");

    let cb = ggez::ContextBuilder::new("game_of_life", "Kenneth <kennethgomezfdz@gmail.com>")
        .window_mode(w_mode)
        .window_setup(w_setup);

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new(1)?;
    event::run(ctx, event_loop, state)
}
