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
    let cmd_func_opt = commands::get_command(&command);
    if let None = cmd_func_opt {
        error(errorf!("command \"{}\" not recognized", command));
    }

    let cmd_func = cmd_func_opt.unwrap();
    if let Err(e) = cmd_func(&args) {
        error(e);
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
