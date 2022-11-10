
pub mod window;
use window::POK8;

use std::{env, process::exit};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("ERROR:  Error parsing cli arguments");
        usage();
        exit(1);
    }
    
    let help_keyword = "help".to_string();
    if args.contains(&help_keyword) {
        usage();
        exit(0);
    }
    
    
    match args.len() {
        
        2 => {
            
            POK8::init(&args[1], None);
        },

        3 => {

            let scale = args[2].parse::<u32>().expect("Could not parse window_scale given. \n See cargo run help for usage.");
            
            POK8::init(&args[1], Some(scale));
        },
        
        _ => {
            // Unreachable case
            println!("Catastrophic failure. Unknown Error.");
            usage();
            exit(1);
        }
    } 
}

fn usage() -> () {
    println!("USAGE (default scale):  cargo run path/to/game");
    println!("USAGE (custom scale):  cargo run path/to/game window_scale");
}