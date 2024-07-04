use crate::msgbox_func::*;

pub fn display_help() {
    let mut help = String::from("Usage:\n\nResetINI <filename> <key_name1> <key_name2> ...");
    help += "\n\n\n";
    help += "<filename>\tINI Filename with path\n";
    help += "<key_name1>\tKey name to reset it's value\n";
    help += "\n\nYou can give as many key names as needed after the filename, separated by a space.";

    info_msgbox(&help, None);
}

pub fn display_string_vector(strvec: &Vec<String>) {
    // For testing. Merge all elements of given string vector and display 
    // it in a message box.
    // Example:
    //      let v = vec!["string1".to_string(), "string2".to_string()]
    //      display_string_vector(&v);
    let message = strvec.join("\n");
    msgbox(&message, None);
}
