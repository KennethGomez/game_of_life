use ggez::graphics::{BlendMode, DrawParam, Drawable, Rect};
use ggez::{graphics, Context, GameResult};

/**
    == == == == == == == == == == == == == == == == ==

    GAME RULES:

    a = alive
    v = neighbor count

        a == false && v == 3 ==> a = true
        a == true && (v < 2 || v > 3) ==> a = false

    == == == == == == == == == == == == == == == == ==
**/

pub struct Grid {
    pub dimensions: Option<Rect>,
    blend_mode: BlendMode,
    size: f32,
    grid: Option<Vec<Vec<bool>>>,
}

impl Grid {
    pub fn update(&self) {
        println!("xd")
    }
}

impl Grid {
    pub fn new(size: f32) -> Self {
        Self {
            dimensions: None,
            blend_mode: BlendMode::Add,
            size,
            grid: None,
        }
    }

    pub fn setup(&mut self) {
        if let Some(dimensions) = self.dimensions {
            let mut grid = Vec::new();

            for _ in 0..((dimensions.w / self.size) as i32) {
                let mut column = Vec::new();

                for _ in 0..((dimensions.h / self.size) as i32) {
                    column.push(true);
                }

                grid.push(column);
            }

            self.grid = Some(grid);
        }
    }
}

impl Drawable for Grid {
    fn draw(&self, ctx: &mut Context, _param: DrawParam) -> GameResult<()> {
        if let Some(grid) = &self.grid {
            for (x, row) in grid.iter().enumerate() {
                for (y, _) in row.iter().enumerate() {
                    let rect = graphics::Rect::new(
                        x as f32 * self.size,
                        y as f32 * self.size,
                        self.size,
                        self.size,
                    );

                    let mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(1.0),
                        rect,
                        [1.0, 1.0, 1.0, 0.25].into(),
                    )?;

                    mesh.draw(ctx, DrawParam::default())?;
                }
            }
        }

        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        self.dimensions
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
