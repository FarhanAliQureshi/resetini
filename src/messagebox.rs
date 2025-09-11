use windows_sys::Win32::UI::WindowsAndMessaging::*;

const MSGBOX_CAPTION: &str = "Reset INI";

fn win_msgbox(message: &String, caption: Option<&String>, style: Option<&u32>) {
    // Prepare a default caption string from constant
    let default_caption = String::from(MSGBOX_CAPTION);
    // If no optional caption is given (None) then use default caption string
    let expanded_caption = caption.unwrap_or(&default_caption);

    // Win32API expect c-style string which are terminated with null character.
    // Therefore, add null character to message and caption strings.
    let c_msg = format!("{}\0", message);
    let c_caption = format!("{}\0", expanded_caption);

    // If msgbox style is not given then use default style of just an OK button
    let expanded_style = style.unwrap_or(&MB_OK).clone();

    // Calling Win32 API is unsafe 
    unsafe {
        // Ignore the return value using underscore
        _ = MessageBoxA(0, c_msg.as_ptr(), c_caption.as_ptr(), 
            expanded_style);
    }
}

pub fn msgbox(message: &String, caption: Option<&String>) {
    win_msgbox(message, caption, Some(&MB_OK));
}

pub fn err_msgbox(message: &String, caption: Option<&String>) {
    let style = MB_OK | MB_ICONERROR;
    win_msgbox(message, caption, Some(&style));
}

pub fn info_msgbox(message: &String, caption: Option<&String>) {
    let style = MB_OK | MB_ICONINFORMATION;
    win_msgbox(message, caption, Some(&style));
}
