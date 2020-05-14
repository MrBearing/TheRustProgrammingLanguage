extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // std::env::args -> std::env::args_os 
    //不正なユニコードを受け取る可能性がある場合はこっちを使う

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
