use rand::Rng;

pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn random() -> Self {
        rand::thread_rng().gen_range(0..4).into()
    }
}

impl From<i32> for Cell {
    fn from(i: i32) -> Self {
        match i {
            1 => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        match self {
            Cell::Dead => Cell::Dead,
            Cell::Alive => Cell::Alive,
        }
    }
}
