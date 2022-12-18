
mod tictactoe;
use tictactoe::tictactoe::*;

fn main() {
    let mut board: Board = Default::default();

    while !board.game_over() {
        board.display();

        let (mut x, mut y, mut legal) = match board.get_play() {
            Ok((x, y, legal)) => (x, y, legal),
            Err(MoveError::BadInput) =>     { println!("Could not parse input, please try again.");             (0, 0, false) },
            Err(MoveError::Occupied) =>     { println!("That square is not empty, please try another one.");    (0, 0, false) },
            Err(MoveError::OutOfBounds) =>  { println!("That square is out of bounds, please try again");       (0, 0, false) },
        };

        while !legal {
            (x, y, legal) = match board.get_play() {
                Ok((x, y, legal)) => (x, y, legal),
                Err(MoveError::BadInput) =>     { println!("Could not parse input, please try again.");             (0, 0, false) },
                Err(MoveError::Occupied) =>     { println!("That square is not empty, please try another one.");    (0, 0, false) },
                Err(MoveError::OutOfBounds) =>  { println!("That square is out of bounds, please try again");       (0, 0, false) },
            };
        }

        board.play(x, y);
    }

    board.display();
    if board.won() {
        println!("{} won!", board.current_player_char());
    }
    else {
        println!("It's a draw");
    }
}
