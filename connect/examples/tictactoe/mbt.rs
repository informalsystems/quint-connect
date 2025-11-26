use crate::game::*;
use quint_connect::*;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(tag = "tag", content = "value")]
enum Square {
    Occupied(Player),
    Empty,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
struct GameState {
    board: BTreeMap<usize, BTreeMap<usize, Square>>,
    #[serde(rename = "nextTurn")]
    next_turn: Player,
}

impl State<TicTacToeDriver> for GameState {
    fn from_driver(driver: &TicTacToeDriver) -> anyhow::Result<Self> {
        let mut board: BTreeMap<usize, BTreeMap<usize, Square>> = BTreeMap::new();
        for (col, x) in driver.game.board.iter().zip(1..) {
            for (cell, y) in col.iter().zip(1..) {
                let square = cell.map(Square::Occupied).unwrap_or(Square::Empty);
                board.entry(y).or_default().insert(x, square);
            }
        }
        Ok(Self {
            board,
            next_turn: driver.game.next_turn,
        })
    }
}

#[derive(Default)]
struct TicTacToeDriver {
    game: TicTacToe,
}

impl Driver for TicTacToeDriver {
    type State = GameState;

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            init => self.game = TicTacToe::default(),
            MoveX(corner?, coordinate?) => match corner.or(coordinate) {
                Some(pos) => self.game.move_to(to_game_pos(pos), Player::X),
                None => self.game.move_to((1, 1), Player::X)
            },
            MoveO(coordinate) => self.game.move_to(to_game_pos(coordinate), Player::O),
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
