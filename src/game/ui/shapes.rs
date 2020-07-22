use web_sys::Path2d;
use crate::game::utils::coord::Coord;
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
    quadrant: u8,
    owner: String
}

impl Circle {
    pub fn new(path: Path2d, quadrant: u8, owner: String) -> Circle {
        Circle { path, quadrant, owner: owner }
    }

    pub fn get_path(&self) -> &Path2d {
        &self.path
    }

    pub fn get_quadrant(&self) -> u8 {
        self.quadrant
    }

    pub fn get_owner(&self) -> &String {
        &self.owner
    }
}
