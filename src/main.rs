use std::env;
use std::path::Path;
use colored::{Colorize};

mod hardware;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}\n{} {}\n\n{}\n{}\t\t{}", "Aether Virtual Machine", "Usage:".green(), "aethervm [COMMAND]".truecolor(0, 255, 255), "Options:".green(), "run".truecolor(0, 255, 255), "Execute and run program from bytecode file")
    }else {
        let arg = &args[1];
        let path = Path::new(&args[2]);
        println!("Arg is {}, arg value is {}", arg, path.display());
    }
    // println!("{:X}", 0x124567890102u64 & 0x000FFFFFFFFF)


}
