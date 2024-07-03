#![windows_subsystem = "windows"]

use windows_sys::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    unsafe {
        MessageBoxA(0, s!("Ansi"), s!("Caption"), MB_OK);
        MessageBoxW(0, w!("Wide"), w!("Caption"), MB_OK);
    }
}
