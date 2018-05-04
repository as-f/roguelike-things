//! Populate a level with mobs

use super::tile::{Terrain, Tile};
use grid::Grid;

pub(super) fn populate(level: Grid<Terrain>) -> Grid<Tile> {
    Grid::new(level.width, level.height, |pos| Tile {
        terrain: level[pos],
        mob_id: None,
    })
}
