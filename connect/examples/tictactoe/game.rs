use serde::Deserialize;

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(tag = "tag")]
pub enum Player {
    X,
    O,
}

pub type Position = (usize, usize);
pub type GameBoard = [[Option<Player>; 3]; 3];

#[derive(Default, Debug)]
pub struct TicTacToe {
    pub board: GameBoard,
}

impl TicTacToe {
    pub fn move_to(&mut self, pos: Position, player: Player) {
        let (x, y) = pos;
        let prev = self.board[y][x].replace(player);
        assert!(prev.is_none(), "moving to ocupied cell");
    }
}
