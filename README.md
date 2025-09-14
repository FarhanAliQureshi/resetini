# Reset INI file  
Resets (to empty) values of keys in an INI file.

## Build Instructions
* [Install Rust](https://www.rust-lang.org/tools/install)
* Clone the repository
* For a release build, run: `cargo build --release`
* For Windows 32-bit static release build, run: `cargo build --config .cargo/win32_static.toml --release`
> [!IMPORTANT]
> Build a Windows executable binary as this program expects to run on Windows.

## How to use?
Usage: `ResetINI <filename> <key_name1> <key_name2> ...`

Where:
| Argument | Notes |
|---|---|
| `<filename>` | INI filename with path |
| `<key_name1>` | Key name to reset it's value |

> [!NOTE]
> If you supply wildcard `*` as key name then values for all keys will be reset to empty.

## External Crates/Libraries  
* `windows-sys` is used to display message boxes (in case of error) on Windows operating systems. It also helps to not splash terminal window when running this program.

## Real world usage
I have been using it for about a year now (since 2024). This utility silently runs in a script when resetting the test environment in a virtual machine. It is a small utility and it runs fast even when other programs are running in parallel.

## License
Copyright © Farhan Ali Qureshi. All rights reserved. See the [MIT LICENSE](LICENSE) file for details.
