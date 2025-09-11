#![windows_subsystem = "windows"]

mod messagebox;
mod process_ini_file;
mod input_validation;
mod gui_io;

use messagebox::*;
use process_ini_file::*;
use input_validation::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !validate_args(&args) {
        return;
    }

    // We can't rely on validate_args() as it can change in future. Therefore, we'll
    // check minimum number of arguments before using them.
    if args.len() < 3 {
        return;
    }
    // First argument is a filename (may include path)
    let filename = args[1].clone();
    // Everything after filename are key names.
    let key_names: Vec<String> = args[2..].to_vec();

    // Read INI file into memory
    let mut file_lines = match read_lines_from_file(&filename) {
        Ok(lines_read) => lines_read,
        Err(e) => {
            err_msgbox(&e, None);
            return;
        },
    };

    // If INI file is empty then no need to process it. Just silently exit.
    if file_lines.len() == 0 {
        return;
    }

    // Reset (delete) values of given keys
    reset_keys_values(&mut file_lines, &key_names);

    // Write modified lines to source INI file
    match write_lines_to_file(&filename, &file_lines) {
        Ok(_) => (),
        Err(e) => {
            // If there was any error during writing to output file then
            // display it to the user.
            err_msgbox(&e, None);
            return;
        },
    };
}
