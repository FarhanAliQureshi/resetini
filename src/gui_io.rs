use crate::messagebox::*;

pub fn display_help() {
    let help = String::from("Usage:

ResetINI <filename> <key_name1> <key_name2> ...

<filename>\tINI Filename with path
<key_name1>\tKey name to reset it's value

You can give as many key names as needed after the filename, separated by a space.

If you supply wildcard '*' (without quotes) as key name then values for all keys will be reset to empty.

https://github.com/FarhanAliQureshi/resetini");

    info_msgbox(&help, None);
}

#[allow(dead_code)]
pub fn display_string_vector(strvec: &Vec<String>) {
    // For testing and debugging. 
    // Merge all elements of given string vector and display 
    // it in a message box.
    // Example:
    //      let v = vec!["string1".to_string(), "string2".to_string()]
    //      display_string_vector(&v);
    let message = strvec.join("\n");
    msgbox(&message, None);
}
