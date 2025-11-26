use serde::Deserialize;

#[derive(Default, Eq, PartialEq, Deserialize, Clone, Copy, Debug)]
#[serde(tag = "tag")]
pub enum Player {
    #[default]
    X,
    O,
}

pub type Position = (usize, usize);
pub type GameBoard = [[Option<Player>; 3]; 3];

#[derive(Default, Debug)]
pub struct TicTacToe {
    pub board: GameBoard,
    pub next_turn: Player,
}

impl TicTacToe {
    pub fn move_to(&mut self, pos: Position, player: Player) {
        assert!(self.next_turn == player, "player out of turn");

        let (x, y) = pos;
        let prev = self.board[y][x].replace(player);
        assert!(prev.is_none(), "moving to ocupied cell");

        self.next_turn = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
