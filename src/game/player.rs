use super::hand::Hand;
use super::gobblet::{Gobblet};

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Hand,
}

impl Player {
    pub fn new(name: String) -> Player {
        let hand = Hand::new(name.clone());
        Player{ name, hand }
    }

    pub fn remove_piece_from_hand(&mut self, hand_section: u8) -> Option<Gobblet> {
        self.hand.remove_piece(hand_section)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}


// Player Tests
#[cfg(test)]
mod tests {
    use super::Player;
    use super::super::gobblet::GobbletSize;

    fn create_player(name: String) -> Player {
        Player::new(name)
    }

    #[test]
    fn remove_piece_from_hand_should_remove_four_pieces_in_order() {
        let mut p = create_player(String::from("Alex"));
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

    #[test]
    fn get_name_should_return_name() {
        let p = create_player(String::from("Alex"));
        let expected = String::from("Alex");
        assert_eq!(p.get_name(), expected);
    }

}