use ggez::{graphics, nalgebra as na};

pub struct Stats {
    pub generation: u32,
    pub alive_cells: u32,
    pub dead_cells: u32,
    dimensions: Option<graphics::Rect>,
    blend_mode: graphics::BlendMode,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            generation: 0,
            alive_cells: 0,
            dead_cells: 0,
            dimensions: None,
            blend_mode: graphics::BlendMode::Add,
        }
    }

    pub fn set_dimensions(&mut self, dimensions: graphics::Rect) {
        self.dimensions = Some(dimensions);
    }

    pub fn set_cells(&mut self, alive: u32, dead: u32) {
        self.alive_cells = alive;
        self.dead_cells = dead;
    }

    pub fn tick(&mut self) {
        self.generation += 1;
    }
}

impl graphics::Drawable for Stats {
    fn draw(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> ggez::GameResult<()> {
        let mut generation_txt = graphics::Text::new(format!("Generation: {}", self.generation));
        let mut cells_text = graphics::Text::new(format!(
            "Alive cells: {} | Dead cells: {}",
            self.alive_cells, self.dead_cells
        ));

        generation_txt.set_font(graphics::Font::default(), graphics::Scale::uniform(22.0));
        cells_text.set_font(graphics::Font::default(), graphics::Scale::uniform(18.0));

        generation_txt.draw(ctx, param)?;
        cells_text.draw(
            ctx,
            (na::Point2::new(param.dest.x, param.dest.y + 30.0),).into(),
        )?;

        Ok(())
    }

    fn dimensions(&self, _ctx: &mut ggez::Context) -> Option<graphics::Rect> {
        self.dimensions
    }

    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {
        if let Some(blend_mode) = mode {
            self.blend_mode = blend_mode;
        }
    }

    fn blend_mode(&self) -> Option<graphics::BlendMode> {
        Some(self.blend_mode)
    }
}
