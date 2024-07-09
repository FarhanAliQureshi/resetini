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
    let file_lines = match read_lines_from_file(&filename) {
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
    let modified_lines = reset_keys_values(&file_lines, &key_names);

    // Write modified lines to source INI file
    match write_lines_to_file(&filename, &modified_lines) {
        Ok(_) => (),
        Err(e) => {
            // If there was any error during writing to output file then
            // display it to the user.
            err_msgbox(&e, None);
            return;
        },
    };
}
