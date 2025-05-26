use crate::config::{config_dir, ensure_dir_exists};
use std::env;
use std::fs;

pub fn run() {
    ensure_dir_exists();
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 || args[0] != "update" {
        eprintln!("Use: task-cli update <id> <new description>");
        return;
    }

    let id = &args[1];
    let new_description = args[2..].join(" ");

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

    json["description"] = serde_json::Value::String(new_description.trim().to_string());

    fs::write(&path, json.to_string()).expect("BUG: Could not write updated task");

    println!("Task updated successfully.");
}
