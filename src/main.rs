#[cfg(windows)]
extern crate winapi;

use winapi::um::debugapi::IsDebuggerPresent;

fn main() {
    unsafe {
        match IsDebuggerPresent() {
            0 => println!("Debugger is not present..."),
            _ => { 
                println!("Debugger detected. Terminating...");
                std::process::exit(0);
            },
        }
    }
    loop {};
}
