use std::error;

mod add;
mod detail;
mod list;

pub use self::add::add_file as add;
pub use self::detail::detail_file as detail;
pub use self::list::list_files as list;

type Command = fn (&[String]) -> Result<(), Box<error::Error>>;
const COMMANDS: &'static [
    (&'static str, &'static str, &'static str, Command)
] = &[
    ("add", "<file> <image>", "Adds file to image.",   add::add_file),
    ("ls",  "<image>", "List files on image.",         list::list_files),
    ("detail",  "<image> <file>", "Show file details", detail::detail_file),
];

pub fn get_command(name: &String) -> Option<Command> {
    for cmd in COMMANDS {
        let &(cmd_name, _, _, cmd_func) = cmd;
        if name == cmd_name {
            return Some(cmd_func);
        }
    }

    None
}

pub fn usages() {
    for &(name, usage, description, _) in COMMANDS {
        println!("\t{} {}: {}", name, usage, description);
    }
}
