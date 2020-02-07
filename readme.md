# Rust FFI Workshop

This is a Rust FFI workshop centered around interop with C/C++, both calling into Rust from C/C++,
and calling into C/C++ from Rust. We will be using MSVC to build a command line tool that
interacts with the Windows Clipboard.

## Getting Setup

While Rust works on Windows, Mac, and Linux, the workshop will have Windows specific content,
so please make sure to bring a Windows laptop or a Windows VM with you that has Rust installed on it (not in the Linux Subsystem for Windows).

You'll need the following things on your Windows machine:
*  Rust (go to https://rustup.rs)
*  Git for Windows
*  VS Code, with the Rust RLS extension , and the C/C++ extension from Microsoft.
*  Visual Studio 2019 with the Desktop development for C++ tools, particularly
    *  VC++ 2019 v142 tools
    *  Windows 10 SDK 10.0.17763.0
*  LLVM 9.0.0 (go to http://releases.llvm.org/download.html and select Windows (64-bit) from the pre-built binaries section)
*  Bindgen
*  After installing Rust, from cmd, run “cargo install bindgen”
*  Set LIBCLANG_PATH as an environment variable pointing to the bin directory of your LLVM install. (see https://rust-lang.github.io/rust-bindgen/requirements.html for details)

## Getting Building

To build, you will first need to launch "x64 Native Tools Command Prompt for VS 2019".
Typically this can be found under the Visual Studio 2019 folder in the start menu.
From this command window, navigate to the [start-here](./start-here) folder and launch VS Code.

In VS Code, type `ctrl+shift+b` to launch build tasks. You will want to run cargo build first, followed by msvc build.
If you have issues with the msvc build, type `ctrl+shift+p` to open the VS Code command palette, and look for
"C/C++: Edit Configurations (UI)". This page may have some automatic suggestions.

The clipboard.exe binary will be output to the folder VS Code was opened from.

## Getting Started

We will implementing parts of a command line tool called clipboard.exe, which interacts with the Windows Clipboard.
Command line parsing is implemented in C++, as well as one basic command, paste. We will implement one more command,
paste to a file, as well as error reporting.

You will need to fill out the `print_clipboard_file` file in [lib.rs](./start-here/clipboard_rs/src/lib.rs), and have errors reported through the
`ReportError` function in [ErrorRecord.hpp](./start-here/ErrorRecord.hpp). [main.cpp](./start-here/main.cpp) has a function `PrintClipboardText`,
which demonstrates the usage of the Clipboard APIs.

Here's a suggested sequence of steps to accomplish this:
    1) Call `print_clipboard_file` from main.cpp (hint: you'll need a function definition in C, not C++).
    2) Translate the filename string to Rust (use std::os::windows::prelude::* and std::ffi::OsString).
    3) Write some bindings for OpenClipboard, CloseClipboard, and GetLastError in bindings.rs.
    4) Write a wrapper struct for OpenClipboard and CloseClipboard (hint: impl the `Drop` trait)
    5) Use the [winapi](https://crates.io/crates/winapi) crate for GetClipboardData, GlobalLock, and GlobalUnlock.
    6) Write wrappers for GetClipboardData and the GlobalLock functions (hint: use lifetimes and references to keep your code safe)
    7) Use bindgen to generate Rust bindings for the ErrorRecord.hpp file (see [build.rs](./start-here/clipboard_rs/build.rs))
    8) Implement the rest of the clipboard.exe commands


## Additional Resources for FFI

* [Rustnomicon on FFI](https://doc.rust-lang.org/nomicon/ffi.html) - lots of good informaton, especially helpful on linking.
* [Rust FFI guid](https://michael-f-bryan.github.io/rust-ffi-guide/) - very helpful guide.
* [winapi docs](https://docs.rs/winapi/0.3.8/winapi/) - bindings for almost all Windows APIs.
* For this project, [Windows Clipboard documentation](https://docs.microsoft.com/en-us/windows/win32/dataxchg/clipboard)


## Additional Resources for Rust in general

* [The Book](https://doc.rust-lang.org/book/): _The_ resource for learning Rust
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/): Learning Rust by going through examples
* [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/): A great way to push yourself to understand Rust's borrower checker and memory model
* [Learn Rust the Dangerous Way](http://cliffle.com/p/dangerust/): For those who really want to understand what is happening with Rust compared to C, this is a great resource.
* [Jon Gjengset’s Rust Streams](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ): The best intermediate to advanced long form content on Rust
* [Ryan's Learn Rust Stream](https://www.youtube.com/watch?v=DWNyZXUC1u4): An early version of part-1 of this workshop