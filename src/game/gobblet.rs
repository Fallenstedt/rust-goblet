#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GobbletSize {
    Tiny,
    Small,
    Medium,
    Large
}

#[derive(Debug, Clone)]
pub struct Gobblet {
    pub size: GobbletSize,
    name: String
}

impl Gobblet {
    pub fn new(size: GobbletSize, name: String) -> Gobblet {
        Gobblet{ size, name }
    }
}

// Player Tests
#[cfg(test)]
mod tests {
    use super::{Gobblet, GobbletSize}; 

    #[test]
    fn new_should_create_gobblet_with_size() {
        let p = Gobblet::new(GobbletSize::Tiny, String::from("Angelica"));

        match p.size {
            GobbletSize::Tiny => assert_eq!(true, true),
            _ => assert_eq!(false, true)
        };
    }
}