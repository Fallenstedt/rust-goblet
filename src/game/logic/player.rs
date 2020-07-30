use crate::game::logic::hand::Hand;
use crate::game::logic::gobblet::Gobblet;
use crate::game::manager::PlayerNumber;

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Hand,
    number: PlayerNumber
}

impl Player {
    pub fn new(name: String, number: PlayerNumber) -> Player {
        let hand = Hand::new(number);
        Player{ name, hand, number }
    }

    pub fn remove_piece_from_hand(&mut self, hand_section: u8) -> Option<Gobblet> {
        self.hand.remove_piece(hand_section)
    }

    pub fn get_player_number(&self) -> &PlayerNumber {
        &self.number
    }
}


// Player Tests
#[cfg(test)]
mod tests {
    use super::Player;
    use crate::game::logic::gobblet::GobbletSize;
    use crate::game::manager::PlayerNumber;
    
    fn create_player(name: String, number: PlayerNumber) -> Player {
        Player::new(name, number)
    }

    #[test]
    fn remove_piece_from_hand_should_remove_four_pieces_in_order() {
        let mut p = create_player(String::from("Alex"), PlayerNumber::One);
        let mut count = 0u32;
        
        loop {
            count += 1;
            let piece = p.remove_piece_from_hand(1);

            let piece = match piece {
                Some(p) => p,
                None => {
                    // There should only be 4 pieces, 
                    // which means 5th access to spot 1 in hand is None
                    assert_eq!(count, 5);
                    break;
                }
            };

            match piece.get_size() {
                &GobbletSize::Large => assert_eq!(count, 1),
                &GobbletSize::Medium => assert_eq!(count, 2),
                &GobbletSize::Small => assert_eq!(count, 3),
                &GobbletSize::Tiny => assert_eq!(count, 4),
            }
            continue;
        }
    }
}