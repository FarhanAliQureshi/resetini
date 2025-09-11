# Reset INI file  
Resets given keys' values in given INI file.

## Build Instructions
* [Install Rust](https://www.rust-lang.org/tools/install)
* Clone the repository
* For a release build, run: `cargo build --release`
* For Windows 32-bit static build, run: `cargo build --config .cargo\win32_static_release.toml --release`
> [!IMPORTANT]
> Build a Windows executable binary as this project expects to run on Windows.

## External Crates/Libraries  
* `windows-sys` is used to display message boxes (in case of error) on Windows operating systems. It also helps to not splash terminal window in automated launch on startup.
