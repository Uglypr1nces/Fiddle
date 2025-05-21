
/*
Project made my Uglyprincess
this is a simple project to make manipulating with files and folders easier

Created at 2025-06-21

File mapping:
src/main.rs - main file
src/lib.rs - library file
src/tools.rs - file manipulation
src/core.rs - general functions and variables
*/

pub mod tools;

use fiddle::*;
use fiddle::tools::FIRST_GREETING;

fn main() {
    check_init();

    println!("{}", FIRST_GREETING);
}
