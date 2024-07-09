pub fn read_lines_from_file(filename: &String) -> Result<Vec<String>, String> {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };

    if filename == "" {
        return Err(String::from("No filename is given"));
    }

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(format!("Error opening file: {}\n\nError:{}", filename, e)),
    };

    // Read all lines (from INI file) into vector of strings
    let buf = BufReader::new(file);
    let lines = buf.lines()
        .map(|l| l.unwrap_or("Could not parse line".to_string()))
        .collect();
    
    Ok(lines)
}

pub fn reset_keys_values(lines: &Vec<String>, keys: &Vec<String>) -> Vec<String> {
    let mut modified_lines: Vec<String> = Vec::new();

    for line in lines {
        // Trim all white spaces from line
        let trimmed = line.trim();
        
        // Ignore empty lines
        if trimmed == "" {
            modified_lines.push(line.clone());
            continue;
        }
        // Ignore comments
        if trimmed.chars().nth(0).unwrap() == '#' {
            modified_lines.push(line.clone());
            continue;
        }
        if trimmed.chars().nth(0).unwrap() == ';' {
            modified_lines.push(line.clone());
            continue;
        }
        // Ignore section declaration
        if trimmed.chars().nth(0).unwrap() == '[' {
            modified_lines.push(line.clone());
            continue;
        }
        // Confirm it's a key value line
        if trimmed.contains("=") {
            // Does the key matches with one of given keys' names
            if key_matches(trimmed, &keys) {
                // If key name matches then reset it's value
                modified_lines.push(reset_key(&line));
            } else {
                // If key name doesn't match then ignore the line
                modified_lines.push(line.clone());
            }
        } else {
            // If line is not a key value then ignore it
            modified_lines.push(line.clone());
        }
    }

    modified_lines
}

fn key_matches(line: &str, keys: &Vec<String>) -> bool {
    let (line_key, _) = match line.split_once("=") {
        None => (line, ""),
        Some(s) => s,
    };

    for key in keys {
        // Ignore text case
        if line_key.to_lowercase() == key.to_lowercase() {
            return true;
        }
    }

    return false;
}

fn reset_key(line: &String) -> String {
    let (key, _) = match line.split_once("=") {
        None => (line.as_str(), ""),
        Some(s) => s,
    };

    key.to_string() + "="
}

pub fn write_lines_to_file(filename: &String, lines: &Vec<String>) -> Result<String, String> {
    use std::fs;
    
    // Flatten Vec<String> into a str slice and write it to the output file.
    // Handle error and return a formatted error message string.
    return match fs::write(filename, lines.join("\n")) {
        Err(e) => Err(format!("Error writing to file: {}\n\nError: {}", filename, e)),
        Ok(_) => Ok(String::new()),
    };
}