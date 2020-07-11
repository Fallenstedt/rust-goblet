use super::hand::Hand;
use super::gobblet::Gobblet;

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Hand,
}

impl Player {
    pub fn new(name: String) -> Player {
        let hand = Hand::new();
        Player{ name, hand }
    }

    pub fn remove_piece_from_hand(&mut self, hand_section: u8) -> Option<Gobblet> {
        self.hand.remove_piece(hand_section)
    }
}
