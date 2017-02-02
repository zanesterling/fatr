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
    let mut args = env::args();
    let name = args.next().unwrap();
    let command: String;
    match args.next() {
        Some(arg) => command = arg,
        None => {
            usage(name);
            process::exit(-1);
        }
    }

    // check for command
    match command.as_ref() {
        "add" => {
            if let Err(e) = add::add_file(&mut args) {
                error(e);
            }
        },
        "ls" => {
            if let Err(e) = list::list_files(&mut args) {
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
    println!("\tls <image>: list files on image.");
}

fn error(err: Box<error::Error>) {
    println!("error: {}", err);
    process::exit(-1);
}
