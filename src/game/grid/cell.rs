use rand::Rng;

pub enum Cell {
    DEAD = 0,
    ALIVE = 1,
}

impl Cell {
    pub fn random() -> Self {
        rand::thread_rng().gen_range(0..10).into()
    }
}

impl From<i32> for Cell {
    fn from(i: i32) -> Self {
        match i {
            1 => Cell::ALIVE,
            _ => Cell::DEAD,
        }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        match self {
            Cell::DEAD => Cell::DEAD,
            Cell::ALIVE => Cell::ALIVE,
        }
    }
}
