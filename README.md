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

To run the unit tests for basic functions of the game

```sh
cargo test
```

To run test for a complete game

```sh
chmod +x tests/test_game.sh
./tests/test_game.sh
```

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