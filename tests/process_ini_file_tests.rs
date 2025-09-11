#![cfg(test)]

#[path ="../src/process_ini_file.rs"]
mod process_ini_file;

use crate::process_ini_file::*;

// *** reset_keys_values()
#[test]
fn test_normal_for_reset_keys_values() {
    let mut input_lines = vec!["key1=value1".to_string()];
    let input_keys = vec!["key1".to_string()];
    let expected_result = "key1=".to_string();

    reset_keys_values(&mut input_lines, &input_keys);
    assert_eq!(input_lines[0], expected_result);
}

#[test]
fn test_empty_input_for_reset_keys_values() {
    let mut input_lines = vec!["".to_string()];
    let input_keys = vec!["key1".to_string()];
    let expected_result = "".to_string();

    reset_keys_values(&mut input_lines, &input_keys);
    assert_eq!(input_lines[0], expected_result);
}

#[test]
fn test_non_existing_key_for_reset_keys_values() {
    let mut input_lines = vec!["key1=value1".to_string()];
    let input_keys = vec!["key2".to_string()];
    let expected_result = "key1=value1".to_string();

    reset_keys_values(&mut input_lines, &input_keys);
    assert_eq!(input_lines[0], expected_result);
}

#[test]
fn test_empty_key_for_reset_keys_values() {
    let mut input_lines = vec!["key1=value1".to_string()];
    let input_keys = vec!["".to_string()];
    let expected_result = "key1=value1".to_string();

    reset_keys_values(&mut input_lines, &input_keys);
    assert_eq!(input_lines[0], expected_result);
}

#[test]
fn test_normal_asterisk_wildcard_for_reset_keys_values() {
    let mut input_lines = vec![
        "key1=value1".to_string(),
        "key2=value2".to_string(),
        "key3=value3".to_string(),
    ];
    let elements_count = input_lines.len();
    let input_keys = vec!["*".to_string()];
    let expected_result = vec![
        "key1=".to_string(),
        "key2=".to_string(),
        "key3=".to_string(),
    ];

    reset_keys_values(&mut input_lines, &input_keys);

    assert_eq!(input_lines.len(), elements_count, "Source should have same number of keys after removing values");
    assert_eq!(input_lines, expected_result);
}

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