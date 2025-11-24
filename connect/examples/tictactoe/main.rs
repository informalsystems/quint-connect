mod game;

#[cfg(test)]
mod mbt;

use game::*;

fn main() {
    let mut game = TicTacToe::default();
    game.move_to((0, 0), Player::X);
    game.move_to((1, 1), Player::O);
}
