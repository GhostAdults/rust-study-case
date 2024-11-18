use std::env;
// collect
use minigrep::{Config, FileConfiguration};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new();
    match config.push_variable(args) {
        Ok(()) => {}
        Err(e) => {
            println!("problem somthing with here:{}", e);
            process::exit(0)
        }
    }
    // b
    if let Err(e) = minigrep::run(&config) {
        println!("app error:{}", e);
        process::exit(0)
    }
}
