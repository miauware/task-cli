use crate::config::config_dir;
use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 || args[0] != "delete" {
        eprintln!("Use: task-cli delete <id>");
        return;
    }
    let id = &args[1];
    let filename = format!("{}.json", id);
    let path = format!("{}/{}", config_dir(), filename);

    if !std::path::Path::new(&path).exists() {
        eprintln!("Task with id '{}' not found.", id);
        return;
    }

    fs::remove_file(&path).expect("BUG: Failed to delete the task");
    println!("Task '{}' successfully deleted.", id);
}
