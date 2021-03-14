use crate::game::grid::Grid;
use crate::game::stats::Stats;

use ggez::graphics::Drawable;
use ggez::{event, graphics, nalgebra as na, timer};

pub struct GameState {
    update_time: u32,
    initialized: bool,
    grid: Grid,
    stats: Stats,
}

impl GameState {
    pub fn new(update_time: u32) -> ggez::GameResult<Self> {
        let grid = Grid::new(10.0);
        let stats = Stats::new();

        Ok(Self {
            update_time,
            initialized: false,
            grid,
            stats,
        })
    }

    fn tick(&mut self) {
        let (alive_cells, dead_cells) = self.grid.update();

        self.stats.tick();
        self.stats.alive_cells = alive_cells;
        self.stats.dead_cells = dead_cells;
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.update_time == 0 {
            self.tick();
        } else {
            while timer::check_update_time(ctx, self.update_time) {
                self.tick();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.22, 0.25, 0.28, 1.0].into());

        let sc_w = graphics::screen_coordinates(ctx).w;
        let sc_h = graphics::screen_coordinates(ctx).h;

        if !self.initialized {
            self.grid
                .initialize(graphics::Rect::new(0.0, 0.0, sc_w, sc_h - 75.0));
            self.stats
                .set_dimensions(graphics::Rect::new(0.0, sc_h - 65.0, sc_w, 75.0));

            self.initialized = true;
        }

        self.grid.draw(ctx, (na::Point2::new(0.0, 0.0),).into())?;
        self.stats
            .draw(ctx, (na::Point2::new(10.0, sc_h - 65.0),).into())?;

        graphics::present(ctx)?;

        Ok(())
    }
}
