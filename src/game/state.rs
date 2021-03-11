use crate::game::grid::Grid;
use ggez::graphics;
use ggez::graphics::{Drawable, Rect, DrawParam};
use ggez::nalgebra as na;
use ggez::{event, Context, GameResult};

pub struct GameState {}

impl GameState {
    pub fn new() -> ggez::GameResult<Self> {
        Ok(Self {})
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        println!("xd");

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let grid = Grid::new(Rect::new(
            0f32,
            0f32,
            graphics::screen_coordinates(ctx).w,
            graphics::screen_coordinates(ctx).h,
        ));

        grid.draw(ctx, DrawParam::default());

        graphics::present(ctx)?;

        Ok(())
    }
}
