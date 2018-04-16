use level::tile::Tile;
use util::grid::{Direction, Pos};

pub struct Player {
    pub pos: Pos,
    pub facing: Direction,
}

impl Player {
    pub fn new(pos: Pos) -> Self {
        Player {
            pos,
            facing: Direction::East,
        }
    }
}