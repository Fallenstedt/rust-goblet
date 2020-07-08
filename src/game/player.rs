use super::hand::Hand;

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
}
