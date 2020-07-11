
#[derive(Debug, Clone)]
struct Coord(u8, u8);


#[derive(Debug, Clone)]
pub enum GobbletSize {
    Tiny,
    Small,
    Medium,
    Large
}

#[derive(Debug, Clone)]
pub struct Gobblet {
    size: GobbletSize,
    coord: Option<Coord>,
}

impl Gobblet {
    pub fn new(size: GobbletSize) -> Gobblet {
        Gobblet{ size, coord: None }
    }
}