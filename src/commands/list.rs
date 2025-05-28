use crate::config::{config_dir, ensure_dir_exists};
use std::fs;
use std::path::PathBuf;
use colored::*; 

pub fn run() {
    ensure_dir_exists();
    let dir = config_dir();

    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("BUG: Failed to read directory")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    entries.sort_by_key(|path| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0)
    });

    if entries.is_empty() {
        println!("No tasks found.");
        return;
    }
    println!("--------------------------------------------------------------");
    println!(
        "{:<12} | {:<6} | {:<12} | {}",
        "|   Status", "Id", "Priority", "Task                  |"
    );
    println!("--------------------------------------------------------------");
    for path in entries {
        let content = fs::read_to_string(&path).expect("BUG: Failed to read task file");
        let json: serde_json::Value = serde_json::from_str(&content).expect("BUG: Invalid JSON");

        // INFO: Extracts the numeric ID from the file name
        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("?").blue();

        let description = json["description"].as_str().unwrap_or("<invalid>");
        let status = if json["done"].as_bool().unwrap_or(false) {
            " Done"
        } else {
            " Todo"
        }.magenta();
         let priority = json["priority"].as_str().unwrap_or("no priority");
        let padded = format!("{:<12}", priority);

        let colored_priority = match priority {
        "low" => padded.replace(priority, &format!("{}", "low".green())),
        "medium" => padded.replace(priority, &format!("{}", "medium".blue())),
        "high" => padded.replace(priority, &format!("{}", "high".red())),
        _ => padded,};
        println!("|   {:<9}| {:<6} | {} | {}",status, id, colored_priority, description);
        println!("--------------------------------------------------------------");

    }
}
