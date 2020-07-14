#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GobbletSize {
    Tiny,
    Small,
    Medium,
    Large
}

#[derive(Debug, Clone)]
pub struct Gobblet {
    size: GobbletSize,
    name: String
}

impl Gobblet {
    pub fn new(size: GobbletSize, name: String) -> Gobblet {
        Gobblet{ size, name }
    }

    pub fn get_size(&self) -> &GobbletSize {
        return &self.size
    }
    
    pub fn get_name(&self) -> String {
        self.name.clone()
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