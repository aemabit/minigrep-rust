use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let config = Config::new(&args);

    println!("file: => {}", config.filename);
    println!("search => {}", config.query);

    Config::run(config);
}
