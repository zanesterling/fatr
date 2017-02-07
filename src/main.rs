use std::env;
use std::error;
use std::process;

#[macro_use] mod utils;

mod commands;
mod fat;

fn main() {
    let mut arg_iter = env::args();
    let name = arg_iter.next().unwrap();
    let command: String;
    match arg_iter.next() {
        Some(arg) => command = arg,
        None => {
            usage(name);
            process::exit(-1);
        }
    }

    // check for command
    let args: Vec<String> = arg_iter.collect();
    match commands::get_command(&command) {
        Some(cmd_func) =>
            if let Err(e) = cmd_func(&args) {
                error(e);
            },

        None => error(errorf!("command \"{}\" not recognized", command)),
    }
}

fn usage(name: String) {
    println!("{}:", name);
    commands::usages();
}

fn error(err: Box<error::Error>) {
    println!("error: {}", err);
    process::exit(-1);
}
