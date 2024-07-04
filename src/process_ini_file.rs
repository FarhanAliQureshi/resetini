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
