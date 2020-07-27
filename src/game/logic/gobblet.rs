use crate::game::manager::PlayerNumber;

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
    player_number: PlayerNumber,
    quadrant: u8
}

impl Gobblet {
    pub fn new(size: GobbletSize, player_number: PlayerNumber, quadrant: u8) -> Gobblet {
        Gobblet{ size, player_number, quadrant }
    }

    pub fn get_size(&self) -> &GobbletSize {
        return &self.size
    }
    
    pub fn get_player_number(&self) -> &PlayerNumber {
        &self.player_number
    }

    pub fn get_quardrant(&self) -> &u8 {
        &self.quadrant
    }
}

// Player Tests
#[cfg(test)]
mod tests {
    use super::{Gobblet, GobbletSize}; 
    use super::PlayerNumber;

    #[test]
    fn new_should_create_gobblet_with_size() {
        let p = Gobblet::new(GobbletSize::Tiny, PlayerNumber::One, 1);

        match p.size {
            GobbletSize::Tiny => assert_eq!(true, true),
            _ => assert_eq!(false, true)
        };
    }
}