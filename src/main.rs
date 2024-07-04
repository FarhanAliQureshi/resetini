#![windows_subsystem = "windows"]

mod msgboxfunc;

use msgboxfunc::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // display_string_vector(&args);
    // return;

    // If no command-line arguments are given then display help and quit
    if args.len() == 1 {
        display_help();
        return;
    }
    // If user asked to display help then display help and quit
    if args.len() > 1 {
        if args[1] == "/?" || args[1].to_lowercase() == "/h" || args[1].to_lowercase() == "-h" {
            display_help();
            return;
        }
    }
    // INI Filename is in args[1]. After that all remaining arguments are key names
    // If user didn't give key names after the filename then give error and exit
    if args.len() == 2 {
        err_msgbox(&String::from("No key names are given."), None);
        return;
    }

    // With above filters, at this point, we are sure that the args length is greater than 1,
    // therefore, we are sure we can access 2nd element.
    let filename = args[1].clone();
    msgbox(&filename, Some(&"Filename".to_string()));

    // We are also sure that args length is greater than 2. Everything after filename
    // are key names.
    let key_names: Vec<String> = args[2..].to_vec();
    display_string_vector(&key_names);

}

fn display_help() {
    let mut help = String::from("Usage:\n\nResetINI <filename> <key_name1> <key_name2> ...");
    help += "\n\n\n";
    help += "<filename>\tINI Filename with path\n";
    help += "<key_name1>\tKey name to reset it's value\n";
    help += "\n\nYou can give as many key names as needed after the filename, separated by a space.";

    info_msgbox(&help, None);
}

fn display_string_vector(strvec: &Vec<String>) {
    // For testing. Merge all elements of given string vector and display 
    // it in a message box.
    let mut message = strvec.join("\n");
    message.push_str("\0"); 
    msgbox(&message, None);
}