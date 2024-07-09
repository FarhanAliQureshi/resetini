use crate::ui_output_funcs::*;
use crate::msgbox_func::*;

pub fn validate_args(args: &Vec<String>) -> bool {
    // If no command-line argument is given then display help and exit
    if args.len() == 1 {
        display_help();
        return false;
    }

    // If user asked for help then display help and exit
    if args.len() > 1 {
        match args[1].to_lowercase().as_str() {
            // Multiple patterns in one guard clause
            "/?" | "-?" | "/h" | "-h" => {
                display_help();
                return false;
            },
            // Ignore rest of cases
            _ => (),
        };
    }

    // INI Filename is in args[1]. After that all remaining arguments are key names
    // If user didn't give any key name then give error and exit
    if args.len() == 2 {
        err_msgbox(&String::from("No key name is given."), None);
        return false;
    }

    // Passed all guard clauses. The user arguments should be good enough to proceed
    return true;
}
