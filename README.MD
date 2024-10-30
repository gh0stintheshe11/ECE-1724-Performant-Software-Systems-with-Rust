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
$ cargo run -- Utility -h
```

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
   Expected Result:
   ```
   ## Search Utility
   ```

2. Search in multiple files:
   ```
   $ cargo run -- Utility tests/*.md tests/recursive/*.md
   ```
   Expected Result:
   ```
   ## Search Utility
   ## Search Utility
   ```

3. Case-insensitive search:
   ```
   $ cargo run -- Utility tests/grep.md -i
   ```
   Expected Result:
   ```
   ## Search Utility
   In this programming assignment, you are expected to implement a command-line utility that
   ```

4. Print line numbers:
   ```
   $ cargo run -- Utility tests/grep.md -n
   ```
   Expected Result:
   ```
   1: ## Search Utility
   ```

5. Invert match:
   ```
   $ cargo run -- Utility tests/grep.md -v
   ```
   Expected Result:
   ```
   In this programming assignment, you are expected to implement a command-line utility that
   searches for a specific pattern in one or multiple files, similar in spirit to the UNIX
   `grep` command.
   ```

6. Recursive search:
   ```
   $ cargo run -- Utility tests -r
   ```
   Expected Result:
   ```
   ## Search Utility
   ## Search Utility
   ```

7. Print filenames:
   ```
   $ cargo run -- Utility tests -r -f
   ```
   Expected Result:
   ```
   tests/recursive/grep.md: ## Search Utility
   tests/grep.md: ## Search Utility
   ```

8. Colored output:
   ```
   $ cargo run -- Utility tests -r -c -f
   ```
   Expected Result:
   ```
   tests/recursive/grep.md: ## Search Utility
   tests/grep.md: ## Search Utility
   ```
   where the matched text is coloured in red.

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

## Unit Tests

To run the test suite for this project, use the following command:

```
$ cargo test
```

This will execute all unit tests and integration tests, ensuring the functionality of the search utility.

This grep-like utility includes a comprehensive suite of unit tests to ensure its functionality across various use cases. Here's a breakdown of what each test covers:

### 1. Basic Search (`test_basic_search`)
- Tests the fundamental search functionality.
- Checks if the utility correctly finds a simple pattern in a file.
- Verifies that non-existent patterns return no results.

### 2. Case-Insensitive Search (`test_case_insensitive_search`)
- Evaluates the case-insensitive search option.
- Confirms that when enabled, the search matches patterns regardless of case.
- Verifies that case-sensitive search works correctly when the option is disabled.

### 3. Line Number Display (`test_print_line_numbers`)
- Tests the line number display feature.
- Ensures that line numbers are correctly prepended to matching lines when the option is enabled.
- Verifies that line numbers are not displayed when the option is disabled.

### 4. Inverted Match (`test_invert_match`)
- Checks the inverted match functionality.
- Confirms that when enabled, the utility returns lines that do not match the pattern.
- Verifies normal (non-inverted) matching behavior when the option is disabled.

### 5. Filename Display (`test_print_filenames`)
- Tests the filename display feature for multi-file searches.
- Ensures that filenames are correctly prepended to results when the option is enabled.
- Verifies that filenames are not displayed when the option is disabled.

### 6. Colored Output (`test_colored_output`)
- Evaluates the colored output feature.
- Checks if matching patterns are highlighted with ANSI color codes when the option is enabled.
- Verifies that no color codes are present in the output when the option is disabled.

### 7. Recursive Search (`test_recursive_search`)
- Tests the recursive directory search functionality.
- Creates a temporary directory structure with multiple files.
- Verifies that all matching files are found when recursive search is enabled.
- Confirms that only top-level files are searched when recursive search is disabled.
- Checks if file paths are correctly reported in the search results.

These tests cover the core functionality of the grep-like utility, ensuring that it behaves correctly under various configurations and search scenarios. They help maintain the reliability and correctness of the tool as we continue to develop and enhance its features.

## Automated Testing Script

The project includes a shell script `run_all_and_compare.sh` that automates testing of the grep utility against a suite of test cases. This script:

1. Extracts test files from `a2-marking-tests.tar.gz`
2. Creates an `outputs` directory to store test results
3. Runs 10 different test commands that verify various features:
   - Basic help output
   - Single file search
   - Multiple file search
   - Wildcard pattern search
   - Case-insensitive search
   - Line number display
   - Inverted match
   - Recursive search
   - Filename display
   - Complex search with multiple options

### Usage

```bash
# Make the script executable
chmod +x run_all_and_compare.sh

# Run the test suite
./run_all_and_compare.sh
```

### Output

The script generates:
- Individual test outputs in the `outputs/` directory (output_1.txt through output_10.txt)
- A `compare.txt` file showing any differences between the actual outputs and expected outputs

### Test Cases

1. `cargo run -- -h`: Tests help message display
2. `cargo run -- Utility tests/grep.md`: Tests basic single file search
3. `cargo run -- Utility tests/grep.md tests/recursive/grep.md`: Tests multiple file search
4. `cargo run -- Utility tests/*.md tests/recursive/*.md`: Tests wildcard pattern matching
5. `cargo run -- Utility tests/grep.md -i`: Tests case-insensitive search
6. `cargo run -- Utility tests/grep.md -n`: Tests line number display
7. `cargo run -- Utility tests/grep.md -v`: Tests inverted match
8. `cargo run -- Utility tests -r`: Tests recursive directory search
9. `cargo run -- Utility tests -r -f`: Tests recursive search with filename display
10. `cargo run -- Utility torch/*.py torch/*/*.py -r -f -i`: Tests complex search with multiple options

The script automatically compares the output of each test case against the expected output stored in `a2-marking-tests/correct-outputs/`.