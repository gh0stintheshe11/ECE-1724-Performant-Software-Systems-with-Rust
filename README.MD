# Rust Search Utility

A command-line utility similar to the UNIX `grep` command, implemented in Rust. This utility searches for specific strings in one or multiple files, with various options for customizing the search behavior.

## Features

- Basic string search in single or multiple files
- Support for wildcard characters in file paths
- Case-insensitive search
- Line number printing
- Inverted match (exclude matching lines)
- Recursive directory search
- Filename printing
- Colored output for matched text

## Usage

```
Usage: grep [OPTIONS] <pattern> <files...>
Options:
-i                Case-insensitive search
-n                Print line numbers
-v                Invert match (exclude lines that match the pattern)
-r                Recursive directory search
-f                Print filenames
-c                Enable colored output
-h, --help        Show help information
```

## Examples

1. Basic search:
   ```
   $ cargo run -- Utility tests/grep.md
   ```

2. Search in multiple files:
   ```
   $ cargo run -- Utility tests/*.md tests/recursive/*.md
   ```

3. Case-insensitive search:
   ```
   $ cargo run -- Utility tests/grep.md -i
   ```

4. Print line numbers:
   ```
   $ cargo run -- Utility tests/grep.md -n
   ```

5. Invert match:
   ```
   $ cargo run -- Utility tests/grep.md -v
   ```

6. Recursive search:
   ```
   $ cargo run -- Utility tests -r
   ```

7. Print filenames:
   ```
   $ cargo run -- Utility tests -r -f
   ```

8. Colored output:
   ```
   $ cargo run -- Utility tests -r -c -f
   ```

## Implementation Notes

- Uses a `Config` struct to store search configuration
- Utilizes external crates:
  - `colored` for colored output
  - `walkdir` for recursive directory traversal
- Error handling is minimal; assumes correct user input and UTF-8 encoded files

## Building and Running

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository.
3. Navigate to the project directory.
4. Build the project:
   ```
   $ cargo build
   ```
5. Run the utility:
   ```
   $ cargo run -- [OPTIONS] <pattern> <files...>
   ```