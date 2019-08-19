use new_minigrep::config::Config;
use std::process;

fn main() {

    let config = Config::new(std::env::args())
        .unwrap_or_else(|error|{
            println!("Error in arguments parsing: {}", error);
            process::exit(1);
        });

    if let Err(e) = new_minigrep::run::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
