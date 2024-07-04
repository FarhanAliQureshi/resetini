use crate::ui_output_funcs::*;
use crate::msgbox_func::*;

pub fn validate_args(args: &Vec<String>) -> bool {
    // If no command-line arguments are given then display help and quit
    if args.len() == 1 {
        display_help();
        return false;
    }
    // If user asked to display help then display help and quit
    if args.len() > 1 {
        if args[1] == "/?" || args[1].to_lowercase() == "/h" || args[1].to_lowercase() == "-h" {
            display_help();
            return false;
        }
    }
    // INI Filename is in args[1]. After that all remaining arguments are key names
    // If user didn't give key names after the filename then give error and exit
    if args.len() == 2 {
        err_msgbox(&String::from("No key names are given."), None);
        return false;
    }

    // Passed all filters then user arguments are valid
    return true;
}
