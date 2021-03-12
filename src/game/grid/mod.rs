use crate::game::grid::cell::Cell;
use ggez::graphics;
use ggez::graphics::Drawable;

pub mod cell;

pub struct Grid {
    pub dimensions: Option<graphics::Rect>,
    blend_mode: graphics::BlendMode,
    size: f32,
    grid: Option<Vec<Vec<Cell>>>,
}

impl Grid {
    pub fn new(size: f32) -> Self {
        Self {
            dimensions: None,
            blend_mode: graphics::BlendMode::Add,
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
                    column.push(Cell::random());
                }

                grid.push(column);
            }

            self.grid = Some(grid);
        }
    }

    /**
        == == == == == == == == == == == == == == == == ==

        GAME RULES:

        a = alive
        v = neighbor count

            a == false && v == 3 ==> a = true
            a == true && (v < 2 || v > 3) ==> a = false

        == == == == == == == == == == == == == == == == ==
    **/

    pub fn update(&self) {
        println!("xd")
    }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Self {
            dimensions: self.dimensions.clone(),
            blend_mode: self.blend_mode.clone(),
            size: self.size,
            grid: self.grid.clone()
        }
    }
}

impl Drawable for Grid {
    fn draw(&self, ctx: &mut ggez::Context, _param: graphics::DrawParam) -> ggez::GameResult<()> {
        if self.grid.is_some() {
            for (x, y, cell) in self.clone() {
                let rect = graphics::Rect::new(
                    x as f32 * self.size,
                    y as f32 * self.size,
                    self.size,
                    self.size,
                );

                let color = match cell {
                    Cell::DEAD => [0.22, 0.25, 0.28, 1.0],
                    Cell::ALIVE => [1.0, 1.0, 1.0, 0.25],
                };

                let stroke = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(1.0),
                    rect,
                    [1.0, 1.0, 1.0, 1.0].into(),
                )?;

                let fill = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    color.into(),
                )?;

                fill.draw(ctx, graphics::DrawParam::default())?;
                stroke.draw(ctx, graphics::DrawParam::default())?;
            }
        }

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

type GridIteratorItem = (usize, usize, Cell);

impl IntoIterator for Grid {
    type Item = GridIteratorItem;
    type IntoIter = GridIterator;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self.grid.expect("Cannot iterate through grid"),
            current_column: 0,
            current_row: 0,
        }
    }
}

pub struct GridIterator {
    grid: Vec<Vec<Cell>>,
    current_column: usize,
    current_row: usize,
}

impl Iterator for GridIterator {
    type Item = GridIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        let new_row = self.current_row + 1;

        if new_row != self.grid[self.current_column].len() {
            self.current_row = new_row;
        } else {
            self.current_row = 0;

            let new_column = self.current_column + 1;

            if new_column != self.grid.len() {
                self.current_column = new_column;
            } else {
                return None
            }
        }

        Some((
            self.current_column,
            self.current_row,
            self.grid[self.current_column][self.current_row].clone(),
        ))
    }
}
