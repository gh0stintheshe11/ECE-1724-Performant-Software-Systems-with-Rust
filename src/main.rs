use reversi::{create_initial_board, print_board, parse_input, apply_move, print_winner, is_valid_move, has_valid_moves};

use std::io::{self, Write};

fn main() {
    let mut board = create_initial_board();
    let mut current_player = 'B';

    loop {
        print_board(&board);
        if !has_valid_moves(&board, current_player) {
            println!("{} player has no valid move.", current_player);
            current_player = if current_player == 'B' { 'W' } else { 'B' };
            if !has_valid_moves(&board, current_player) {
                println!("{} player has no valid move.", current_player);
                break;
            }
            continue;
        }

        print!("Enter move for colour {} (RowCol): ", current_player);
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if let Some((row, col)) = parse_input(input) {
            if is_valid_move(&board, current_player, row, col) {
                apply_move(&mut board, current_player, row, col);
                current_player = if current_player == 'B' { 'W' } else { 'B' };
            } else {
                println!("Invalid move. Try again.");
            }
        } else {
            println!("Invalid input. Try again.");
        }
    }
    print_winner(&board);
}

