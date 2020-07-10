use super::player::Player;
use super::board::Board;

#[derive(Debug)]
pub struct Manager {
    player1: Player,
    player2: Player,
    board: Board,
}

impl Manager {
    pub fn new(name1: String, name2: String) -> Manager {
        let player1 = Player::new(name1);
        let player2 = Player::new(name2);
        let board = Board::new();
        Manager{ player1, player2, board }
    }
    
}
