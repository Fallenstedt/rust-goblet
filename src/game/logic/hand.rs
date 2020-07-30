use std::collections::HashMap;
use crate::game::logic::gobblet::{Gobblet, GobbletSize};
use crate::game::utils::PlayerNumber;

#[derive(Debug)]
pub struct Hand {
    state: HashMap<u8, Vec<Gobblet>>,
}

impl Hand {
    pub fn new(number: PlayerNumber) -> Hand {
        let mut state = HashMap::new();
        
        for i in 1..4 {
            let mut group = Vec::with_capacity(4);
            group.push(Gobblet::new(GobbletSize::Tiny, number, i));
            group.push(Gobblet::new(GobbletSize::Small, number, i));
            group.push(Gobblet::new(GobbletSize::Medium, number, i));
            group.push(Gobblet::new(GobbletSize::Large, number, i));

            state.insert(i, group);
        }
        Hand { state }
    }

    pub fn remove_piece(&mut self, s: u8) -> Option<Gobblet> {
        let hand_section = self.state.get_mut(&s);

        match hand_section {
            Some(pieces) => pieces.pop(),
            None => None
        }
    }
}