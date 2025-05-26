use crate::config::{config_dir, ensure_dir_exists};
use std::env;
use std::fs;

pub fn run() {
    ensure_dir_exists();
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 || args[0] != "mark" {
        eprintln!("Use: task-cli mark <id>");
        return;
    }
    let id = &args[1];
    let path = format!("{}/{}.json", config_dir(), id);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Task with ID '{}' not found.", id);
            return;
        }
    };

    let mut json: serde_json::Value =
        serde_json::from_str(&content).expect("BUG: Failed to parse JSON");
    json["done"] = serde_json::Value::Bool(true);

    fs::write(&path, json.to_string()).expect("BUG: Could not write updated task");
    println!("Task marked as done.");
}

