use ggez::graphics;
use ggez::graphics::Drawable;
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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let mut text = graphics::Text::new("Game of Life");
        text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));

        let tx = graphics::screen_coordinates(ctx).w / 2.0 - (text.width(ctx) / 2) as f32;
        let ty = graphics::screen_coordinates(ctx).h / 2.0 - (text.height(ctx) / 2) as f32;

        text.draw(ctx, (na::Point2::new(tx, ty),).into())?;

        graphics::present(ctx)?;
        Ok(())
    }
}
