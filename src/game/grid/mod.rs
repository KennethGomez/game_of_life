use crate::game::grid::cell::Cell;
use ggez::graphics;

pub mod cell;

pub struct Grid {
    dimensions: Option<graphics::Rect>,
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

    pub fn initialize(&mut self, dimensions: graphics::Rect) {
        self.dimensions = Some(dimensions);

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

    /**
        == == == == == == == == == == == == == == == == ==

        GAME RULES:

        a = alive
        v = neighbor count

            a == false && v == 3 ==> a = true
            a == true && (v < 2 || v > 3) ==> a = false

        == == == == == == == == == == == == == == == == ==
    **/

    pub fn update(&mut self) -> (u32, u32) {
        let mut alive: u32 = 0;
        let mut dead: u32 = 0;

        if self.grid.is_some() {
            let mut grid = self.grid.clone().unwrap();

            for (x, y, cell) in self.clone() {
                let cell = match (cell, self.get_neighbor_count(x, y)) {
                    (Cell::Alive, n) if n < 2 || n > 3 => Cell::Dead,
                    (Cell::Alive, n) if n == 3 || n == 2 => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (c, _) => c,
                };

                grid[x][y] = cell.clone();

                match cell {
                    Cell::Dead => dead += 1,
                    Cell::Alive => alive += 1
                }
            }

            self.grid = Some(grid)
        }

        (alive, dead)
    }

    fn get_neighbor_count(&mut self, x: usize, y: usize) -> i32 {
        let mut count = 0;

        if let Some(grid) = &self.grid {
            for x_off in vec![-1, 0, 1] {
                for y_off in vec![-1, 0, 1] {
                    let nx = x as i32 + x_off;
                    let ny = y as i32 + y_off;

                    if nx < 0 || nx >= grid.len() as i32 {
                        continue;
                    }

                    if ny < 0 || ny >= grid[nx as usize].len() as i32 {
                        continue;
                    }

                    if nx == x as i32 && ny == y as i32 {
                        continue;
                    }

                    count += grid[nx as usize][ny as usize].clone() as i32;
                }
            }
        }

        count
    }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Self {
            dimensions: self.dimensions.clone(),
            blend_mode: self.blend_mode.clone(),
            size: self.size,
            grid: self.grid.clone(),
        }
    }
}

impl graphics::Drawable for Grid {
    fn draw(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> ggez::GameResult<()> {
        if self.grid.is_some() {
            let mut mb = graphics::MeshBuilder::new();

            for (x, y, cell) in self.clone() {
                let bounds = graphics::Rect::new(
                    x as f32 * self.size,
                    y as f32 * self.size,
                    self.size,
                    self.size,
                );

                mb.rectangle(
                    graphics::DrawMode::Fill(graphics::FillOptions::default()),
                    bounds,
                    match cell {
                        Cell::Dead => [0.22, 0.25, 0.28, 1.0].into(),
                        Cell::Alive => [1.0, 1.0, 1.0, 0.25].into(),
                    },
                )
                .rectangle(
                    graphics::DrawMode::Stroke(graphics::StrokeOptions::default()),
                    bounds,
                    [1.0, 1.0, 1.0, 0.25].into(),
                );
            }

            mb.build(ctx)?.draw(ctx, param)?;
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
                return None;
            }
        }

        Some((
            self.current_column,
            self.current_row,
            self.grid[self.current_column][self.current_row].clone(),
        ))
    }
}
