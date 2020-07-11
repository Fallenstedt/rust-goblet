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
}

impl Gobblet {
    pub fn new(size: GobbletSize) -> Gobblet {
        Gobblet{ size }
    }
}