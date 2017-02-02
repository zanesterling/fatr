use std::env;
use std::error;
use std::process;
use std::collections::hash_map::HashMap;

mod add;
mod detail;
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


type Command = fn (&[String]) -> Result<(), Box<error::Error>>;
fn get_commands() -> HashMap<&'static str, Command> {
    let mut map = HashMap::new();
    map.insert("add",    add::add_file as Command);
    map.insert("ls",     list::list_files as Command);
    map.insert("detail", detail::detail_file as Command);
    map
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
    match get_commands().get(&command.as_ref()) {
        Some(cmd_func) =>
            if let Err(e) = cmd_func(&args) {
                error(e);
            },

        None => errorf!("command \"{}\" not recognized", command),
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
