use crate::game::*;
use quint_connect::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(tag = "tag", content = "value")]
enum Square {
    Occupied(Player),
    Empty,
}

#[derive(Deserialize, Debug)]
struct SpecBoard(HashMap<usize, HashMap<usize, Square>>);

impl From<SpecBoard> for GameBoard {
    fn from(value: SpecBoard) -> Self {
        let mut board = Self::default();
        for (x, col) in value.0 {
            for (y, square) in col {
                if let Square::Occupied(player) = square {
                    board[y - 1][x - 1] = Some(player);
                }
            }
        }
        board
    }
}

#[derive(Default)]
struct TicTacToeDriver {
    game: TicTacToe,
}

impl Driver for TicTacToeDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            init => self.game = TicTacToe::default(),
            MoveX(corner: Position?, coordinate: Position?) => {
                match corner.or(coordinate).map(to_game_pos) {
                    Some(pos) => self.game.move_to(pos, Player::X),
                    None => self.game.move_to((1, 1), Player::X),
                }
            }
            MoveO(coordinate) => self.game.move_to(to_game_pos(coordinate), Player::O)
        })
    }
}

fn to_game_pos(pos: Position) -> Position {
    let (x, y) = pos;
    (x - 1, y - 1)
}

#[quint_run(spec = "examples/tictactoe/tictactoe.qnt", max_samples = 1)]
fn test_tictactoe() -> impl Driver {
    TicTacToeDriver::default()
}
