
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


use fiddle::*;
use std::io::stdin;

const OPTIONS:[&str; 3] = ["change file/folder name", "search for data in file/folder", "get file/folder information"];

fn main() {
    let mut user_choice:String = String::new();

    for option in OPTIONS.iter(){
        println!("{}",option);
    }

    println!("What do you need to do? (1,2,3)");

    stdin().read_line(&mut user_choice)
        .ok()
        .expect("Couldnt save save user choice");

    
}
