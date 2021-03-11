use ggez::graphics::{BlendMode, DrawParam, Drawable, Rect};
use ggez::{graphics, Context, GameResult, nalgebra as na};

pub struct Grid {
    dimensions: Rect,
    blend_mode: BlendMode,
}

impl Grid {
    pub fn new(dimensions: Rect) -> Grid {
        Grid {
            dimensions,
            blend_mode: BlendMode::Add,
        }
    }
}

impl Drawable for Grid {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let mut text = graphics::Text::new("Game of Life");
        text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));

        let tx = graphics::screen_coordinates(ctx).w / 2.0 - (text.width(ctx) / 2) as f32;
        let ty = graphics::screen_coordinates(ctx).h / 2.0 - (text.height(ctx) / 2) as f32;

        text.draw(ctx, (na::Point2::new(tx, ty),).into())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.dimensions)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        if let Some(blend_mode) = mode {
            self.blend_mode = blend_mode;
        }
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        Some(self.blend_mode)
    }
}
