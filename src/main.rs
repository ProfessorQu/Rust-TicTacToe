pub mod tictactoe;
pub mod minimax;

use std::io::{stdin, stdout, Write};

use tictactoe::tictactoe::*;
use minimax::minimax::*;

fn main() {
    print!("Please select mode (vs/ai): ");
    stdout().flush().expect("Failed to flush output");

    let mut mode_string = String::new();
    stdin().read_line(&mut mode_string).expect("Failed to read mode");
    let mode = mode_string.trim();

    let mut board: Board = Default::default();

    // ==============================
    // Player vs player
    // ==============================
    if mode == "vs" {
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
    // ==============================
    // Against AI
    // ==============================
    else if mode == "ai" {
        print!("What do you want to play as (x/o): ");
        stdout().flush().expect("Failed to flush output");

        let mut player_string = String::new();
        stdin().read_line(&mut player_string).expect("Failed to read player");
        let player = player_string.trim();

        if player != "x" && player != "o" {
            println!("Unknown player.");
            return
        }

        while !board.game_over() {
            if (board.x_turn && player == "x") || (!board.x_turn && player == "o") {
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
            else {
                let (x, y) = get_best_move(&mut board);
    
                board.play(x, y);
            }
        }

        board.display();
        if board.won() {
            println!("{} won!", board.current_player_char());
        }
        else {
            println!("It's a draw");
        }
    }
    else {
        println!("Unknown gamemode.");
        return
    }
}
