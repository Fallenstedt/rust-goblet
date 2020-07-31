pub mod coord;


#[derive(Debug, Clone, Copy)]
pub enum PlayerNumber {
    One,
    Two
}

pub fn player_number_match(shape_owner: &PlayerNumber, current_turn: &PlayerNumber) -> bool {
    return std::mem::discriminant(shape_owner) == std::mem::discriminant(current_turn)
}