use web_sys::Path2d;
use crate::game::utils::coord::Coord;
use crate::game::utils::gobblet_size::GobbletSize;
#[derive(Debug, Clone)]
pub struct Rectangle {
    path: Path2d,
    coord: Coord,
}

impl Rectangle {
    pub fn new(path: Path2d, coord: Coord) -> Rectangle {
        Rectangle { path, coord }
    }

    pub fn get_path(&self) -> &Path2d {
        &self.path
    }

    pub fn get_coord(&self) -> &Coord {
        &self.coord
    }
}

#[derive(Debug, Clone)]
pub struct Circle {
    path: Path2d,
    size: GobbletSize,
}

impl Circle {
    pub fn new(path: Path2d, size: GobbletSize) -> Circle {
        Circle { path, size }
    }

    pub fn get_path(&self) -> &Path2d {
        &self.path
    }

    pub fn get_size(&self) -> &GobbletSize {
        &self.size
    }
}
