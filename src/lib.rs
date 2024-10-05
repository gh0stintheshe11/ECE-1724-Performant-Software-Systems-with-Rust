pub const SIZE: usize = 8;
pub type Board = [[char; SIZE]; SIZE];

// Parse input (like "cd") into row and column indices
pub fn parse_input(input: &str) -> Option<(usize, usize)> {
    if input.len() != 2 {
        return None;
    }

    let row = input.chars().nth(0)?.to_ascii_lowercase();
    let col = input.chars().nth(1)?.to_ascii_lowercase();

    // Check if row and column are within 'a' to 'h'
    if row < 'a' || row > 'h' || col < 'a' || col > 'h' {
        return None;
    }

    Some((row as usize - 'a' as usize, col as usize - 'a' as usize))
}

// Create the initial game board
pub fn create_initial_board() -> Board {
    let mut board = [['.'; SIZE]; SIZE];
    board[3][3] = 'W';
    board[3][4] = 'B';
    board[4][3] = 'B';
    board[4][4] = 'W';
    board
}

// Print the board
pub fn print_board(board: &Board) {
    println!("  abcdefgh");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", (b'a' + i as u8) as char);
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}

// Check if a move is valid
pub fn is_valid_move(board: &Board, player: char, row: usize, col: usize) -> bool {
    if board[row][col] != '.' {
        return false;
    }

    let opponent = if player == 'B' { 'W' } else { 'B' };

    // Direction vectors for N, S, W, E, NW, NE, SW, SE
    let directions = [
        (-1, 0), (1, 0), (0, -1), (0, 1),
        (-1, -1), (-1, 1), (1, -1), (1, 1),
    ];

    // Check in all directions
    for &(dr, dc) in &directions {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut found_opponent = false;

        while r >= 0 && r < SIZE as isize && c >= 0 && c < SIZE as isize {
            let cur = board[r as usize][c as usize];
            if cur == opponent {
                found_opponent = true;
            } else if cur == player && found_opponent {
                return true;
            } else {
                break;
            }
            r += dr;
            c += dc;
        }
    }
    false
}

// Apply a move and flip opponent pieces
pub fn apply_move(board: &mut Board, player: char, row: usize, col: usize) {
    board[row][col] = player;

    let opponent = if player == 'B' { 'W' } else { 'B' };

    let directions = [
        (-1, 0), (1, 0), (0, -1), (0, 1),
        (-1, -1), (-1, 1), (1, -1), (1, 1),
    ];

    for &(dr, dc) in &directions {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut to_flip = Vec::new();

        while r >= 0 && r < SIZE as isize && c >= 0 && c < SIZE as isize {
            let cur = board[r as usize][c as usize];
            if cur == opponent {
                to_flip.push((r as usize, c as usize));
            } else if cur == player {
                for (fr, fc) in to_flip {
                    board[fr][fc] = player;
                }
                break;
            } else {
                break;
            }
            r += dr;
            c += dc;
        }
    }
}

// Check if the player has valid moves
pub fn has_valid_moves(board: &Board, player: char) -> bool {
    for row in 0..SIZE {
        for col in 0..SIZE {
            if is_valid_move(board, player, row, col) {
                return true;
            }
        }
    }
    false
}

// Count the number of black and white pieces
pub fn count_pieces(board: &Board) -> (usize, usize) {
    let mut black_count = 0;
    let mut white_count = 0;

    for row in board.iter() {
        for &cell in row.iter() {
            if cell == 'B' {
                black_count += 1;
            } else if cell == 'W' {
                white_count += 1;
            }
        }
    }
    (black_count, white_count)
}

// Print the winner or if it's a draw
pub fn print_winner(board: &Board) {
    let (black_count, white_count) = count_pieces(board);

    if black_count > white_count {
        println!("Black wins by {} points!", black_count - white_count);
    } else if white_count > black_count {
        println!("White wins by {} points!", white_count - black_count);
    } else {
        println!("Draw!");
    }
}