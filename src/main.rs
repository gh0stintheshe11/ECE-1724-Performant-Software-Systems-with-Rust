// Import necessary libraries
use clap::{Arg, ArgAction, Command}; // For parsing command-line arguments
use colored::*; // For colored output
use regex::RegexBuilder; // For building regular expressions
use std::fs::File; // For file operations
use std::io::{self, BufRead, BufReader}; // For reading files
use walkdir::WalkDir; // For recursive directory traversal

// Define a struct to hold the configuration options
struct Config {
    pattern: String,        // search pattern
    files: Vec<String>,     // list of files or directories to search
    ignore_case: bool,      // perform case-insensitive search
    line_number: bool,      // print line numbers
    invert_match: bool,     // invert the match (show non-matching lines)
    recursive: bool,        // search recursively in directories
    print_filename: bool,   // print filenames
    colored_output: bool,   // use colored output
}

fn main() {
    // Set up the command-line interface using clap
    let matches = Command::new("grep")
        .override_usage("grep [OPTIONS] <pattern> <files...>")
        .help_template("{usage-heading}{usage}\n{all-args}")
        // Define command-line arguments
        .arg(
            Arg::new("pattern")
                .required(true)
                .hide(true)
        )
        .arg(
            Arg::new("files")
                .required(true)
                .hide(true)
                .num_args(1..)
        )
        .arg(
            Arg::new("ignore_case")
                .short('i')
                .help("Case-insensitive search")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("line_number")
                .short('n')
                .help("Print line numbers")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("invert_match")
                .short('v')
                .help("Invert match (exclude lines that match the pattern)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .help("Recursive directory search")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("print_filename")
                .short('f')
                .help("Print filenames")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("colored_output")
                .short('c')
                .help("Enable colored output")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Extract the search pattern and files from command-line arguments
    let pattern = matches.get_one::<String>("pattern").unwrap().to_string();
    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    // Create a Config struct with all the options
    let config = Config {
        pattern,
        files,
        ignore_case: matches.get_flag("ignore_case"),
        line_number: matches.get_flag("line_number"),
        invert_match: matches.get_flag("invert_match"),
        recursive: matches.get_flag("recursive"),
        print_filename: matches.get_flag("print_filename"),
        colored_output: matches.get_flag("colored_output"),
    };

    // Get the list of files to search
    let files_to_search = get_files(&config.files, config.recursive);
    let mut results = Vec::new();

    // Search in each file
    for file in files_to_search {
        if let Err(err) = search_in_file(&file, &config, &mut results) {
            eprintln!("Error reading {}: {}", file, err);
        }
    }

    // Print the results
    for line in results {
        println!("{}", line);
    }
}

// Function to get the list of files to search
fn get_files(files: &[String], recursive: bool) -> Vec<String> {
    let mut file_list = Vec::new();
    for file_pattern in files {
        if recursive {
            // Use WalkDir for recursive search
            for entry in WalkDir::new(file_pattern) {
                if let Ok(entry) = entry {
                    if entry.file_type().is_file() {
                        file_list.push(entry.path().display().to_string());
                    }
                }
            }
        } else {
            // Use std::fs for non-recursive search
            if let Ok(metadata) = std::fs::metadata(file_pattern) {
                if metadata.is_file() {
                    file_list.push(file_pattern.to_string());
                } else if metadata.is_dir() {
                    if let Ok(entries) = std::fs::read_dir(file_pattern) {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                                    file_list.push(entry.path().display().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    file_list
}

// Function to search for the pattern in a single file
fn search_in_file(
    filename: &str,
    config: &Config,
    results: &mut Vec<String>,
) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Build the regex pattern
    let regex = RegexBuilder::new(&regex::escape(&config.pattern))
        .case_insensitive(config.ignore_case)
        .build()
        .unwrap();

    // Iterate through each line in the file
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let is_match = regex.is_match(&line);

        // Determine if the line should be included based on invert_match
        let matched = if config.invert_match { !is_match } else { is_match };

        if matched {
            let mut output_line = String::new();

            // Add filename if required
            if config.print_filename {
                output_line.push_str(&format!("{}: ", filename));
            }

            // Add line number if required
            if config.line_number {
                output_line.push_str(&format!("{}: ", index + 1));
            }

            // Add colored output if required
            if config.colored_output {
                let line_colored = regex
                    .replace_all(&line, |caps: &regex::Captures| {
                        caps[0].red().to_string()
                    })
                    .to_string();
                output_line.push_str(&line_colored);
            } else {
                output_line.push_str(&line);
            }

            // Add the formatted line to the results
            results.push(output_line);
        }
    }
    Ok(())
}


// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_file(content: &str) -> (TempDir, String) {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("grep.md");
        fs::write(&file_path, content).unwrap();
        (dir, file_path.to_str().unwrap().to_string())
    }

    fn get_test_content() -> String {
        "## Search Utility\nIn this programming assignment, you are expected to implement a command-line utility that\nsearches for a specific pattern in one or multiple files, similar in spirit to the UNIX\n`grep` command.".to_string()
    }

    #[test]
    fn test_basic_search() {
        let content = get_test_content();
        let (_dir, file_path) = create_test_file(&content);
        
        let config = Config {
            pattern: "Utility".to_string(),
            files: vec![file_path.clone()],
            ignore_case: false,
            line_number: false,
            invert_match: false,
            recursive: false,
            print_filename: false,
            colored_output: false,
        };
        
        let mut results = Vec::new();
        search_in_file(&file_path, &config, &mut results).unwrap();
        
        assert_eq!(results, vec!["## Search Utility"]);

        // Test for non-matching pattern
        let config_no_match = Config {
            pattern: "NonExistentPattern".to_string(),
            ..config
        };
        
        let mut results_no_match = Vec::new();
        search_in_file(&file_path, &config_no_match, &mut results_no_match).unwrap();
        
        assert!(results_no_match.is_empty());
    }

    #[test]
    fn test_case_insensitive_search() {
        let content = get_test_content();
        let (_dir, file_path) = create_test_file(&content);
        
        // Test case-insensitive search (on)
        let config_insensitive = Config {
            pattern: "utility".to_string(),
            files: vec![file_path.clone()],
            ignore_case: true,
            line_number: false,
            invert_match: false,
            recursive: false,
            print_filename: false,
            colored_output: false,
        };
        
        let mut results_insensitive = Vec::new();
        search_in_file(&file_path, &config_insensitive, &mut results_insensitive).unwrap();
        
        assert_eq!(results_insensitive, vec!["## Search Utility", "In this programming assignment, you are expected to implement a command-line utility that"]);

        // Test case-sensitive search (off)
        let config_sensitive = Config {
            pattern: "utility".to_string(),
            files: vec![file_path.clone()],
            ignore_case: false,
            line_number: false,
            invert_match: false,
            recursive: false,
            print_filename: false,
            colored_output: false,
        };
        
        let mut results_sensitive = Vec::new();
        search_in_file(&file_path, &config_sensitive, &mut results_sensitive).unwrap();
        
        assert_eq!(results_sensitive, vec!["In this programming assignment, you are expected to implement a command-line utility that"]);
    }

    #[test]
    fn test_print_line_numbers() {
        let content = get_test_content();
        let (_dir, file_path) = create_test_file(&content);
        
        // Test with line numbers (on)
        let config_with_numbers = Config {
            pattern: "Utility".to_string(),
            files: vec![file_path.clone()],
            ignore_case: false,
            line_number: true,
            invert_match: false,
            recursive: false,
            print_filename: false,
            colored_output: false,
        };
        
        let mut results_with_numbers = Vec::new();
        search_in_file(&file_path, &config_with_numbers, &mut results_with_numbers).unwrap();
        
        assert_eq!(results_with_numbers, vec!["1: ## Search Utility"]);

        // Test without line numbers (off)
        let config_without_numbers = Config {
            line_number: false,
            ..config_with_numbers
        };
        
        let mut results_without_numbers = Vec::new();
        search_in_file(&file_path, &config_without_numbers, &mut results_without_numbers).unwrap();
        
        assert_eq!(results_without_numbers, vec!["## Search Utility"]);
    }

    #[test]
    fn test_invert_match() {
        let content = get_test_content();
        let (_dir, file_path) = create_test_file(&content);
        
        // Test inverted match (on)
        let config_inverted = Config {
            pattern: "Utility".to_string(),
            files: vec![file_path.clone()],
            ignore_case: false,
            line_number: false,
            invert_match: true,
            recursive: false,
            print_filename: false,
            colored_output: false,
        };
        
        let mut results_inverted = Vec::new();
        search_in_file(&file_path, &config_inverted, &mut results_inverted).unwrap();
        
        assert_eq!(results_inverted, vec![
            "In this programming assignment, you are expected to implement a command-line utility that",
            "searches for a specific pattern in one or multiple files, similar in spirit to the UNIX",
            "`grep` command."
        ]);

        // Test normal match (off)
        let config_normal = Config {
            invert_match: false,
            ..config_inverted
        };
        
        let mut results_normal = Vec::new();
        search_in_file(&file_path, &config_normal, &mut results_normal).unwrap();
        
        assert_eq!(results_normal, vec!["## Search Utility"]);
    }

    #[test]
    fn test_print_filenames() {
        let content = get_test_content();
        let (_dir, file_path) = create_test_file(&content);
        
        // Test with filename printing (on)
        let config_with_filename = Config {
            pattern: "Utility".to_string(),
            files: vec![file_path.clone()],
            ignore_case: false,
            line_number: false,
            invert_match: false,
            recursive: false,
            print_filename: true,
            colored_output: false,
        };
        
        let mut results_with_filename = Vec::new();
        search_in_file(&file_path, &config_with_filename, &mut results_with_filename).unwrap();
        
        assert!(results_with_filename[0].starts_with(&file_path));
        assert!(results_with_filename[0].ends_with("## Search Utility"));

        // Test without filename printing (off)
        let config_without_filename = Config {
            print_filename: false,
            ..config_with_filename
        };
        
        let mut results_without_filename = Vec::new();
        search_in_file(&file_path, &config_without_filename, &mut results_without_filename).unwrap();
        
        assert_eq!(results_without_filename, vec!["## Search Utility"]);
    }

    #[test]
    fn test_colored_output() {
        let content = get_test_content();
        let (_dir, file_path) = create_test_file(&content);
        
        // Test with colored output (on)
        let config_colored = Config {
            pattern: "Utility".to_string(),
            files: vec![file_path.clone()],
            ignore_case: false,
            line_number: false,
            invert_match: false,
            recursive: false,
            print_filename: false,
            colored_output: true,
        };
        
        let mut results_colored = Vec::new();
        search_in_file(&file_path, &config_colored, &mut results_colored).unwrap();
        
        assert!(results_colored[0].contains("\x1b["));  // ANSI escape code for colored output

        // Test without colored output (off)
        let config_not_colored = Config {
            colored_output: false,
            ..config_colored
        };
        
        let mut results_not_colored = Vec::new();
        search_in_file(&file_path, &config_not_colored, &mut results_not_colored).unwrap();
        
        assert!(!results_not_colored[0].contains("\x1b["));  // No ANSI escape code
    }

    #[test]
    fn test_recursive_search() {
        let temp_dir = TempDir::new().unwrap();
        let tests_dir = temp_dir.path().join("tests");
        let recursive_dir = tests_dir.join("recursive");
        let file1_path = tests_dir.join("grep.md");
        let file2_path = recursive_dir.join("grep.md");

        std::fs::create_dir_all(&recursive_dir).unwrap();
        std::fs::write(&file1_path, "## Search Utility").unwrap();
        std::fs::write(&file2_path, "## Search Utility").unwrap();

        // Test get_files function
        let config_recursive = Config {
            pattern: "Utility".to_string(),
            files: vec![tests_dir.to_str().unwrap().to_string()],
            ignore_case: false,
            line_number: false,
            invert_match: false,
            recursive: true,
            print_filename: true,
            colored_output: false,
        };

        let files_recursive = get_files(&config_recursive.files, config_recursive.recursive);
        assert_eq!(files_recursive.len(), 2, "Expected 2 files in recursive search, found {}", files_recursive.len());
        assert!(files_recursive.contains(&file1_path.to_str().unwrap().to_string()), "Missing file: {:?}", file1_path);
        assert!(files_recursive.contains(&file2_path.to_str().unwrap().to_string()), "Missing file: {:?}", file2_path);

        // Test search_in_file function
        let mut results_recursive = Vec::new();
        for file in &files_recursive {
            search_in_file(file, &config_recursive, &mut results_recursive).unwrap();
        }

        assert_eq!(results_recursive.len(), 2, "Expected 2 results in recursive search, found {}", results_recursive.len());
        assert!(results_recursive.iter().any(|r| r.contains(&format!("{}: ## Search Utility", file1_path.to_str().unwrap()))), 
                "Missing result for file: {:?}", file1_path);
        assert!(results_recursive.iter().any(|r| r.contains(&format!("{}: ## Search Utility", file2_path.to_str().unwrap()))), 
                "Missing result for file: {:?}", file2_path);

        // Test non-recursive search
        let config_non_recursive = Config {
            recursive: false,
            ..config_recursive
        };

        let files_non_recursive = get_files(&config_non_recursive.files, config_non_recursive.recursive);
        assert_eq!(files_non_recursive.len(), 1, "Expected 1 file in non-recursive search, found {}", files_non_recursive.len());
        assert!(files_non_recursive.contains(&file1_path.to_str().unwrap().to_string()), "Missing file: {:?}", file1_path);

        let mut results_non_recursive = Vec::new();
        for file in &files_non_recursive {
            search_in_file(file, &config_non_recursive, &mut results_non_recursive).unwrap();
        }

        assert_eq!(results_non_recursive.len(), 1, "Expected 1 result in non-recursive search, found {}", results_non_recursive.len());
        assert!(results_non_recursive[0].contains(&format!("{}: ## Search Utility", file1_path.to_str().unwrap())), 
                "Incorrect result for non-recursive search");
    }

}

