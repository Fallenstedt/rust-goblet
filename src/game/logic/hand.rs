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
            group.push(Gobblet::new(GobbletSize::Tiny, number));
            group.push(Gobblet::new(GobbletSize::Small, number));
            group.push(Gobblet::new(GobbletSize::Medium, number));
            group.push(Gobblet::new(GobbletSize::Large, number));

            state.insert(i, group);
        }
        Hand { state }
    }

    pub fn remove_piece(&mut self, section: u8) -> Option<Gobblet> {
        match self.state.get_mut(&section) {
            Some(pieces) => pieces.pop(),
            None => None
        }
    }

    pub fn add_piece(&mut self, gobblet: Gobblet, section: u8) {
         self.state.get_mut(&section).unwrap().push(gobblet);
    }
}