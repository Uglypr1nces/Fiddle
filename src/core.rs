/*
File for general functions and variables

Layout:

core.rs
    -Variables
        - greeting
    -Functions
        - core_init_test() - test function to check if the file is initialized


*/

pub const FIRST_GREETING:&str =  r"
 __      __       .__                                                    .___         __                     
/  \    /  \ ____ |  |   ____  ____   _____   ____     _____ _____     __| _/____   _/  |_  ____             
\   \/\/   // __ \|  | _/ ___\/  _ \ /     \_/ __ \   /     \\__  \   / __ |/ __ \  \   __\/  _ \            
 \        /\  ___/|  |_\  \__(  <_> )  Y Y  \  ___/  |  Y Y  \/ __ \_/ /_/ \  ___/   |  | (  <_> )           
  \__/\  /  \___  >____/\___  >____/|__|_|  /\___  > |__|_|  (____  /\____ |\___  >  |__|  \____/ /\  /\  /\ 
       \/       \/          \/            \/     \/        \/     \/      \/    \/                \/  \/  \/ 
                                                                                                  \
___________.__    .___  .___.__        ._.                                                                   
\_   _____/|__| __| _/__| _/|  |   ____| |                                                                  
 |    __)  |  |/ __ |/ __ | |  | _/ __ \ |                                                                   
 |     \   |  / /_/ / /_/ | |  |_\  ___/\|                                                                   
 \___  /   |__\____ \____ | |____/\___  >_                                                                   
     \/            \/    \/           \/\/  made by Uglyprincess                                                              
";

pub fn core_init_test(){
    println!("core file initialized");
}