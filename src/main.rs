mod config;
mod commands;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        match args[1].as_str() {
            "add"    => commands::add::run(),
            "delete" => commands::delete::run(),
            "list"   => commands::list::run(),
            "mark"   => commands::mark::run(),
            "update" => commands::update::run(),
            cmd => println!("Command not found: {}", cmd),
        }
    } else {
        commands::list::run();
    }
}

