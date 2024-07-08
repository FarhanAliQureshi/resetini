#![windows_subsystem = "windows"]

mod msgbox_func;
mod process_ini_file;
mod validation_funcs;
mod ui_output_funcs;

use msgbox_func::*;
use process_ini_file::*;
use validation_funcs::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !validate_args(&args) {
        return;
    }

    // After user arguments filters, at this point, we are sure that the args length 
    // is greater than 1, therefore, we are sure we can access 2nd element.
    let filename = args[1].clone();
    // We are also sure that args length is greater than 2. Everything after filename
    // are key names.
    let key_names: Vec<String> = args[2..].to_vec();

    // Read INI file into memory
    let file_lines = match read_lines_from_file(&filename) {
        Some(lines_read) => lines_read,
        None => {
            err_msgbox(&format!("Error reading file: {}", filename), None);
            return;
        },
    };

    // If INI file is empty then no need to process it. Just silently exit.
    if file_lines.len() == 0 {
        return;
    }

    // Reset (delete) values of given keys
    let modified_lines = reset_keys_values(&file_lines, &key_names);

    // Write modified lines to source INI file
    write_lines_to_file(&filename, &modified_lines);
}
