use rand::Rng;

pub enum Cell {
    DEAD = 0,
    ALIVE = 1
}

impl Cell {
    pub fn random() -> Self {
        rand::thread_rng().gen_range(0..2).into()
    }
}

impl From<i32> for Cell {
    fn from(i: i32) -> Self {
        match i {
            1 => Cell::ALIVE,
            _ => Cell::DEAD
        }
    }
}