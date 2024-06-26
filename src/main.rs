use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = grep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    if let Err(err) = grep::run(config){
        eprintln!("Problem when running: {err}");
        process::exit(2);
    }
}
