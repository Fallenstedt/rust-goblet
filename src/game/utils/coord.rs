

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

// Player Tests
#[cfg(test)]
mod tests {
    use super::Coord; 

    #[test]
    fn new_should_create_coord_with_row_column() {
        let p = Coord::new(1, 2);
        
        assert_eq!(p.get_row(), &1);
        assert_eq!(p.get_column(), &2);
    }
}
