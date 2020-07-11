
#[derive(Debug, Clone)]
pub struct Coord(u8, u8);

impl Coord {
    pub fn new(row: u8, column: u8) -> Coord {
        Coord(row, column)
    }
    pub fn get_row(&self) -> &u8 {
        &self.0
    }
    pub fn get_column(&self) -> &u8 {
        &self.1
    }
}