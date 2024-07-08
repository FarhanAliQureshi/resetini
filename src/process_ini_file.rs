pub fn read_lines_from_file(filename: &String) -> Option<Vec<String>> {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };

    if filename == "" {
        return None;
    }

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            return None;
        },
    };

    let buf = BufReader::new(file);
    let lines = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    
    Some(lines)
}

pub fn reset_keys_values(lines: &Vec<String>, keys: &Vec<String>) -> Vec<String> {
    let mut modified_lines: Vec<String> = Vec::new();

    for line in lines {
        // Trim all whie spaces from line
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
    let (line_key, _) = line.split_once("=").unwrap();

    for key in keys {
        // Ignore text case
        if line_key.to_lowercase() == key.to_lowercase() {
            return true;
        }
    }

    return false;
}

fn reset_key(line: &String) -> String {
    let (key, _) = line.split_once("=").unwrap();

    key.to_string() + "="
}

pub fn write_lines_to_file(filename: &String, lines: &Vec<String>) {
    use std::fs;
    
    fs::write(filename, lines.join("\n")).expect("");
}