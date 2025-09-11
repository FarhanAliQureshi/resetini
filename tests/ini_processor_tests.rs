#![cfg(test)]

#[path ="../src/ini_processor.rs"]
mod ini_processor;

use crate::ini_processor::*;

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