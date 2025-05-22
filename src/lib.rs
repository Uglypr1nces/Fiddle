pub mod core;
pub mod tools;


/* 
tools.rs
    -Functions
    -Variables
*/

pub use tools::tools_init_test;
/*
core.rs
    -Functions
    -Variables
*/

pub use core::core_init_test;
pub use core::FIRST_GREETING;


pub fn check_init() {
    tools_init_test();
    core_init_test();
}