/*
File for general functions and variables

Layout:

core.rs
    -Variables
        - greeting
    -Functions
        - core_init_test() - test function to check if the file is initialized


*/

pub const FIRST_GREETING: &str = "Hello, world!";


pub fn core_init_test(){
    println!("core file initialized");
}