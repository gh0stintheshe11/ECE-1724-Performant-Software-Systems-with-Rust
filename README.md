# Reversi Board Game

This is a simple implementation of the Reversi board game in Rust. The game is played on an 8x8 board, with two players (Black and White) taking turns to place their pieces on the board. The goal is to flip the opponent's pieces to your color.

## Game Rules

1. The game is played on an 8x8 board.
2. Each player starts with 2 pieces placed in the center of the board.
3. Players take turns placing their pieces on the board.
4. A move is valid if it flips at least one opponent's piece.
5. The game ends when both players have no valid moves.

## Running the Game

To build the game

```sh
cargo build --release
```

To run the game

```sh
cargo run
```

## Testing

To run test for a complete game

```sh
chmod +x tests/test_game.sh
./tests/test_game.sh
```

## Unit Tests

This project includes a comprehensive set of unit tests to ensure the correct functionality of a Reversi game implementation. 

To run the unit tests for basic functions of the game

```sh
cargo test
```

The tests cover various aspects of the game logic and board manipulation. Here's a breakdown of what is being tested:

### 1. Input Parsing (`test_parse_input`)
- Validates the correct parsing of user input (e.g., "cd" to (2, 3)).
- Ensures that valid inputs within the range 'a' to 'h' are correctly converted to board coordinates.
- Verifies that invalid inputs (out of range, incorrect format, extra spaces) are properly rejected.

### 2. Initial Board Setup (`test_create_initial_board`)
- Checks that the initial game board is created with the correct starting position:
  - White pieces at d4 and e5
  - Black pieces at e4 and d5

### 3. Move Validation (`test_is_valid_move`)
- Tests the `is_valid_move` function with different scenarios:
  - Confirms valid moves for both Black and White players in various directions.
  - Verifies that moves to occupied spaces are correctly identified as invalid.

### 4. Move Application (`test_apply_move`)
- Ensures that when a valid move is applied:
  - The new piece is correctly placed on the board.
  - Opponent pieces are properly flipped according to Reversi rules.

### 5. Piece Counting (`test_count_pieces`)
- Verifies the accurate counting of black and white pieces on the board:
  - Checks the initial board state (2 black, 2 white).
  - Confirms correct piece count after applying a move.

### 6. Available Moves Check (`test_has_valid_moves`)
- Tests the `has_valid_moves` function to ensure:
  - Both players have valid moves at the start of the game.
  - After a move is made, the opponent still has valid moves available.

These tests collectively ensure that the core game logic for Reversi is functioning correctly, covering input handling, board management, move validation and application, and game state assessment. This test suite provides a solid foundation for the game's reliability and adherence to Reversi rules.

## Project Structure

```
Reversi-Board-Game/
├── Cargo.toml          # Project configuration file
├── src/
│   ├── main.rs         # Main entry point and logic of the game
│   └── lib.rs          # Game functions
└── tests/
    └── lib_test.rs     # Unit tests for the game logic
    └── test_game.sh    # Test script for the game
    └── test_input.txt  # Test input file for the game
    └── expect_output.txt # Expected output file for the game
```