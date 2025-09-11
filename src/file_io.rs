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

pub fn write_lines_to_file(filename: &String, lines: &Vec<String>) -> Result<String, String> {
    use std::fs;
    
    // Flatten Vec<String> into a str slice and write it to the output file.
    // Handle error and return a formatted error message string.
    return match fs::write(filename, lines.join("\n")) {
        Err(e) => Err(format!("Error writing to file: {}\n\nError: {}", filename, e)),
        Ok(_) => Ok(String::new()),
    };
}