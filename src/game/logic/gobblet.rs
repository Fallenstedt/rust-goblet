use crate::game::utils::gobblet_size::GobbletSize;

#[derive(Debug, Clone)]
pub struct Gobblet {
    size: GobbletSize,
    name: String,
    quadrant: u8
}

impl Gobblet {
    pub fn new(size: GobbletSize, name: String, quadrant: u8) -> Gobblet {
        Gobblet{ size, name, quadrant }
    }

    pub fn get_size(&self) -> &GobbletSize {
        return &self.size
    }
    
    pub fn get_name(&self) -> &String {
        &self.name
    }


    pub fn get_quardrant(&self) -> &u8 {
        &self.quadrant
    }
}

// Player Tests
#[cfg(test)]
mod tests {
    use super::{Gobblet, GobbletSize}; 

    #[test]
    fn new_should_create_gobblet_with_size() {
        let p = Gobblet::new(GobbletSize::Tiny, String::from("Angelica"), 1);

        match p.size {
            GobbletSize::Tiny => assert_eq!(true, true),
            _ => assert_eq!(false, true)
        };
    }
}