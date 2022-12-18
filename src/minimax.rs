pub mod minimax {
    use crate::tictactoe::tictactoe::*;

    pub fn get_best_move(board: &mut Board) -> (usize, usize) {
        let mut best_val;
        match board.x_turn {
            true =>  best_val = i32::MIN,
            false => best_val = i32::MAX
        }

        let mut best_x = 3;
        let mut best_y = 3;

        for y in 0..3 {
            for x in 0..3 {
                if board.empty(x, y) {
                    board.play(x, y);
                    let move_val = minimax(board, 0);
                    board.unplay(x, y);

                    if board.x_turn && move_val > best_val {
                        best_y = y;
                        best_x = x;
                        best_val = move_val;
                    }
                    else if !board.x_turn && move_val < best_val {
                        best_y = y;
                        best_x = x;
                        best_val = move_val;
                    }
                }
            }
        }

        println!("Value for current move: {}", best_val);

        (best_x, best_y)
    }

    fn minimax(board: &mut Board, depth: i32) -> i32 {
        if board.draw() {
            match board.x_turn {
                true => return -100,
                false => return 100
            }
        }
        else if board.won() {
            match board.x_turn {
                true => return 100 - depth,
                false => return -100 + depth
            }
        }

        if board.x_turn {
            let mut best = 0;

            for y in 0..3 {
                for x in 0..3 {
                    if board.empty(x, y) {
                        board.play(x, y);
                        best += minimax(board, depth + 1);
                        board.unplay(x, y);
                    }
                }
            }

            best
        }
        else {
            let mut best = 0;

            for y in 0..3 {
                for x in 0..3 {
                    if board.empty(x, y) {
                        board.play(x, y);
                        best += minimax(board, depth + 1);
                        board.unplay(x, y);
                    }
                }
            }

            best
        }
    }
}