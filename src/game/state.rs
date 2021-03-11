use crate::game::grid::Grid;
use ggez::graphics::{DrawParam, Drawable, Rect};
use ggez::{event, graphics, timer, Context, GameResult};

pub struct GameState {
    update_time: u32,
}

impl GameState {
    pub fn new(update_time: u32) -> ggez::GameResult<Self> {
        Ok(Self { update_time })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, self.update_time) {
            println!("xd");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.22, 0.25, 0.28, 1.0].into());

        let mut grid = Grid::new(
            Rect::new(
                0f32,
                0f32,
                graphics::screen_coordinates(ctx).w,
                graphics::screen_coordinates(ctx).h,
            ),
            16f32,
        );

        grid.setup();
        grid.draw(ctx, DrawParam::default())?;

        graphics::present(ctx)?;

        Ok(())
    }
}
