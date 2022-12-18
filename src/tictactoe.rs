pub mod tictactoe {
    use std::io::{stdin, stdout, Write};

    pub enum MoveError {
        BadInput,
        OutOfBounds,
        Occupied,
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum Cell {
        Empty, X, O
    }

    const NAMES: [char; 3] = [' ', 'X', 'O'];

    const ROWS: [[(usize, usize); 3]; 8] = [
        [(0, 0), (0, 1), (0, 2)], // Top row
        [(1, 0), (1, 1), (1, 2)], // Middle row
        [(2, 0), (2, 1), (2, 2)], // Bottom row
        
        [(0, 0), (1, 0), (2, 0)], // Left column
        [(0, 1), (1, 1), (2, 1)], // Middle column
        [(0, 2), (1, 2), (2, 2)], // Right column

        [(0, 0), (1, 1), (2, 2)], // Left-right diagonal
        [(2, 0), (1, 1), (0, 2)], // Right-left diagonal
    ];

    pub struct Board {
        pub board: [[Cell; 3]; 3],
        pub x_turn: bool,
    }

    impl Board {
        pub fn current_player_char(&self) -> char
        {
            match self.x_turn {
                true => 'X',
                false => 'O'
            }
        }

        fn current_player_cell(&self) -> Cell
        {
            match self.x_turn {
                true => Cell::X,
                false => Cell::O
            }
        }

        pub fn switch_turn(&mut self) {
            self.x_turn = !self.x_turn;
        }

        pub fn display(&self) {
            println!("-------");

            for y in 0..3 {
                for x in 0..3 {
                    let name = NAMES[self.board[y][x] as usize];
                    print!("|{}", name);
                }
                println!("|");
                println!("-------");
            }
        }
        
        pub fn won(&self) -> bool {
            let player = self.current_player_cell();
            for [(x0, y0), (x1, y1), (x2, y2)] in ROWS {
                if self.board[y0][x0] == player && self.board[y1][x1] == player && self.board[y2][x2] == player {
                    return true
                }
            }

            false
        }

        pub fn draw(&self) -> bool {
            if self.won() {
                return false
            }
            else {
                for y in 0..3 {
                    for x in 0..3 {
                        if self.empty(x, y) {
                            return false
                        }
                    }
                }
            }

            true
        }

        pub fn game_over(&self) -> bool {
            self.draw() || self.won()
        }

        fn in_bounds(&self, int: usize) -> bool {
            int < 3
        }

        fn empty(&self, x: usize, y:usize) -> bool {
            self.board[y][x] == Cell::Empty
        }

        fn legal_move(&self, x: usize, y: usize) -> bool
        {
            self.in_bounds(x) && self.in_bounds(y) && self.empty(x, y)
        }

        pub fn get_play(&self) -> Result<(usize, usize, bool), MoveError>
        {
            print!("Enter x coordinate of an empty square: ");
            stdout().flush().expect("Failed to flush output");

            let mut x_string = String::new();
            stdin().read_line(&mut x_string).expect("Failed to read input");

            let x_trimmed = x_string.trim();
            let x = match x_trimmed.parse::<usize>() {
                Err(_) => return Err(MoveError::BadInput),
                Ok(number) => match self.in_bounds(number) {
                    true => number,
                    false => return Err(MoveError::OutOfBounds)
                }
            };
            
            print!("Enter y coordinate of an empty square: ");
            stdout().flush().expect("Failed to flush output");

            let mut y_string = String::new();
            stdin().read_line(&mut y_string).expect("Failed to read input");

            let y_trimmed = y_string.trim();
            let y = match y_trimmed.parse::<usize>() {
                Err(_) => return Err(MoveError::BadInput),
                Ok(number) => match self.in_bounds(number) {
                    true => number,
                    false => return Err(MoveError::OutOfBounds)
                }
            };

            if !self.empty(x, y) {
                Err(MoveError::Occupied)
            }
            else{
                Ok((x, y, self.legal_move(x, y)))
            }
        }

        pub fn play(&mut self, x: usize, y: usize)
        {
            self.board[y][x] = self.current_player_cell();
        }
    }
}

use tictactoe::*;

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                [ Cell::Empty,  Cell::Empty,    Cell::Empty ],
                [ Cell::Empty,  Cell::Empty,    Cell::Empty ],
                [ Cell::Empty,  Cell::Empty,    Cell::Empty ]
            ],
            x_turn: true
        }
    }
}