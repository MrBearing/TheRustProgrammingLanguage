extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    // std::env::args_os //不正なユニコードを受け取る可能性がある場合

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}",config.querry);
    println!("In file {}",config.filename);

    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);

        process::exit(1);
    }
}
