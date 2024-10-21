// Import necessary libraries
use clap::{Arg, ArgAction, Command}; // For parsing command-line arguments
use colored::*; // For colored output
use glob::glob; // For file pattern matching
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
        .version("1.0")
        .about("Search for patterns in files")
        .override_usage("Usage: grep [OPTIONS] <pattern> <files...>")
        // Define command-line arguments
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true),
        )
        .arg(
            Arg::new("files")
                .help("The files or directories to search in")
                .required(true)
                .num_args(1..),
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
                let entry = entry.unwrap();
                if entry.file_type().is_file() {
                    file_list.push(entry.path().display().to_string());
                }
            }
        } else {
            // Use glob for non-recursive search
            for entry in glob(file_pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.is_file() {
                            file_list.push(path.display().to_string());
                        }
                    }
                    Err(e) => eprintln!("Glob error: {:?}", e),
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
