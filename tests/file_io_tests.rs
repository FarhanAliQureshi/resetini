#![cfg(test)]

#[path ="../src/file_io.rs"]
mod file_io;

use crate::file_io::*;

// *** write_lines_to_file()
#[test]
fn test_normal_for_write_lines_to_file() {
    let filename = "test_write.ini".to_string();
    let lines = vec!["key1=value1".to_string()];

    match write_lines_to_file(&filename, &lines) {
        Ok(_) => (),
        Err(e) => panic!("Got error from write_lines_to_file(): {}", e),
    };

    let output_data = match std::fs::read_to_string(&filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error reading file {}: {}", &filename, e),
    };

    match std::fs::remove_file(&filename) {
        Ok(_) => (),
        Err(e) => panic!("Error deleting file {}: {}", &filename, e),
    };

    assert_eq!(lines[0], output_data);
}

// *** read_lines_from_file()
#[test]
fn test_normal_for_read_lines_from_file() {
    let filename = "test_read.ini".to_string();
    let test_data = "key1=value1".to_string();

    match std::fs::write(&filename, &test_data) {
        Ok(_) => (),
        Err(e) => panic!("Error writing to file {}: {}", &filename, e),
    };

    let file_lines = match read_lines_from_file(&filename) {
        Ok(lines_read) => lines_read,
        Err(e) => panic!("Got error from read_lines_from_file(): {}", e),
    };

    match std::fs::remove_file(&filename) {
        Ok(_) => (),
        Err(e) => panic!("Error deleting file {}: {}", &filename, e),
    };

    assert_eq!(test_data, file_lines[0]);
}