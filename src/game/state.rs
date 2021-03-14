use crate::game::grid::Grid;
use ggez::graphics::Drawable;
use ggez::{event, graphics, timer, nalgebra as na};

pub struct GameState {
    update_time: u32,
    grid: Grid,
}

impl GameState {
    pub fn new(update_time: u32) -> ggez::GameResult<Self> {
        let grid = Grid::new(10.0);

        Ok(Self { update_time, grid })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.update_time == 0 {
            self.grid.update();
        } else {
            while timer::check_update_time(ctx, self.update_time) {
                self.grid.update();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.22, 0.25, 0.28, 1.0].into());

        self.grid.setup_if_needed(ctx);
        self.grid.draw(ctx, (na::Point2::new(0.0, 0.0),).into())?;

        graphics::present(ctx)?;

        Ok(())
    }
}
