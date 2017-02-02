use std::env;
use std::error;
use std::process;

mod add;
mod fat;
mod list;

macro_rules! errorf {
    ($fmt:expr, $($arg:expr),*) => {
        error(From::from(format!(
            $fmt,
            $( $arg ),*
        )));
    }
}

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
    match command.as_ref() {
        "add" => {
            if let Err(e) = add::add_file(&args) {
                error(e);
            }
        },
        "ls" => {
            if let Err(e) = list::list_files(&args) {
                error(e);
            }
        },

        _ => {
            errorf!("command \"{}\" not recognized", command);
        },
    }
}

fn usage(name: String) {
    println!("{}:", name);
    println!("\tadd <file> <image>: Adds file to image.");
    println!("\tls <image> [flags]: list files on image.");
}

fn error(err: Box<error::Error>) {
    println!("error: {}", err);
    process::exit(-1);
}
