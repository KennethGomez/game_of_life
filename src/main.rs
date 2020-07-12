use ggez::event;

use game_of_life::game::state::GameState;

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("game_of_life", "Kenneth <kennethgomezfdz@gmail.com>");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
