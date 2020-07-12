#[derive(Debug, Clone)]
pub enum GobbletSize {
    Tiny,
    Small,
    Medium,
    Large
}

#[derive(Debug, Clone)]
pub struct Gobblet {
    pub size: GobbletSize,
}

impl Gobblet {
    pub fn new(size: GobbletSize) -> Gobblet {
        Gobblet{ size }
    }
}

// Player Tests
#[cfg(test)]
mod tests {
    use super::{Gobblet, GobbletSize}; 

    #[test]
    fn new_should_create_gobblet_with_size() {
        let p = Gobblet::new(GobbletSize::Tiny);

        match p.size {
            GobbletSize::Tiny => assert_eq!(true, true),
            _ => assert_eq!(false, true)
        };
    }
}