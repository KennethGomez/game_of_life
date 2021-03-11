use crate::game::grid::Grid;
use ggez::graphics::Drawable;
use ggez::{event, graphics, timer};

pub struct GameState {
    update_time: u32,
    grid: Grid,
}

impl GameState {
    pub fn new(update_time: u32) -> ggez::GameResult<Self> {
        let grid = Grid::new(16.0);

        Ok(Self { update_time, grid })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        while timer::check_update_time(ctx, self.update_time) {
            self.grid.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.22, 0.25, 0.28, 1.0].into());

        if self.grid.dimensions == None {
            self.grid.dimensions = Some(graphics::Rect::new(
                0.0,
                0.0,
                graphics::screen_coordinates(ctx).w,
                graphics::screen_coordinates(ctx).h,
            ));

            self.grid.setup();
        }

        self.grid.draw(ctx, graphics::DrawParam::default())?;

        graphics::present(ctx)?;

        Ok(())
    }
}
