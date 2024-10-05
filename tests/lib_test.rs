// Import the necessary functions and types from the lib.rs module
use reversi::{parse_input, create_initial_board, is_valid_move, apply_move, count_pieces, has_valid_moves};


// Test the parse_input function with valid inputs
#[test]
fn test_parse_input() {
    assert_eq!(parse_input("cd"), Some((2, 3)));  // Row c (2), Column d (3)
    assert_eq!(parse_input("ah"), Some((0, 7)));  // Row a (0), Column h (7)
    assert_eq!(parse_input("hh"), Some((7, 7)));  // Row h (7), Column h (7)
    
    // Invalid inputs
    assert_eq!(parse_input("i8"), None);  // Invalid row (i)
    assert_eq!(parse_input("a9"), None);  // Invalid column (9)
    assert_eq!(parse_input("1a"), None);  // Incorrect format
    assert_eq!(parse_input("aa "), None); // Extra space
}

// Test the create_initial_board function for correct initial placement
#[test]
fn test_create_initial_board() {
    let board = create_initial_board();
    assert_eq!(board[3][3], 'W'); // Initial white piece at d4
    assert_eq!(board[3][4], 'B'); // Initial black piece at e4
    assert_eq!(board[4][3], 'B'); // Initial black piece at d5
    assert_eq!(board[4][4], 'W'); // Initial white piece at e5
}

// Test is_valid_move with different board states and positions
#[test]
fn test_is_valid_move() {
    let board = create_initial_board();

    // Valid move for Black at c4 (upwards direction)
    assert!(is_valid_move(&board, 'B', 2, 3));  // c4

    // Invalid move for Black at d4 (occupied by White)
    assert!(!is_valid_move(&board, 'B', 3, 3)); // d4

    // Valid move for White at c5 (downwards direction)
    assert!(is_valid_move(&board, 'W', 2, 4));  // c5
}

// Test the apply_move function for correct piece flipping
#[test]
fn test_apply_move() {
    let mut board = create_initial_board();

    // Apply a valid move for Black at c4
    apply_move(&mut board, 'B', 2, 3);  // c4

    // Check if the move was applied and pieces flipped correctly
    assert_eq!(board[2][3], 'B'); // Black piece at c4
    assert_eq!(board[3][3], 'B'); // White piece at d4 should have flipped to Black
}

// Test the count_pieces function for correct counting of black and white pieces
#[test]
fn test_count_pieces() {
    let board = create_initial_board();
    assert_eq!(count_pieces(&board), (2, 2)); // Start with 2 Black, 2 White

    let mut board = board;
    apply_move(&mut board, 'B', 2, 3);  // Apply move at c4
    assert_eq!(count_pieces(&board), (4, 1)); // Now 4 Black, 1 White
}

// Test the has_valid_moves function for both players
#[test]
fn test_has_valid_moves() {
    let board = create_initial_board();
    assert!(has_valid_moves(&board, 'B')); // Black has valid moves at the start
    assert!(has_valid_moves(&board, 'W')); // White has valid moves at the start

    // Example: If Black makes the first move at c4, White should still have valid moves
    let mut board = board;
    apply_move(&mut board, 'B', 2, 3); // Apply move at c4
    assert!(has_valid_moves(&board, 'W')); // White should still have valid moves
}