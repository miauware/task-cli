use crate::config::{config_dir, ensure_dir_exists};
use std::env;
use std::fs::{self, File};
use std::io::Write;

pub fn run() {
    ensure_dir_exists();
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 3 || args[0] != "add" {
        eprintln!("Use: task-cli add <low|medium|high> <task description>");
        return;
    }

    let prioridade = args[1].to_lowercase();

    if !["low", "medium", "high"].contains(&prioridade.as_str()) {
        eprintln!("Invalid Priority. Use: low, medium or high.");
        return;
    }

    let descricao = args[2..].join(" ");

    // INFO: Determines the next sequential number for the file name
    let dir = config_dir();
    let next_id = next_task_id(&dir);

    let filename = format!("{}/{}.json", dir, next_id);

    let todo = serde_json::json!({
        "description": descricao.trim(),
        "done": false,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "priority": prioridade
    });

    // INFO: Write the task in the JSON file
    let mut file = File::create(&filename).expect("BUG: Failure when creating the task file");
    file.write_all(todo.to_string().as_bytes())
        .expect("BUG: Failure to write the task in the file");

    println!("Task added successfully.");
}


fn next_task_id(dir: &str) -> u32 {
    let paths = fs::read_dir(dir).unwrap_or_else(|_| panic!("BUG: Failure to read directory '{}'", dir));
    let mut max_id = 0;

    for path in paths.flatten() {
        if let Some(filename) = path.path().file_stem() {
            if let Some(stem_str) = filename.to_str() {
                if let Ok(n) = stem_str.parse::<u32>() {
                    max_id = max_id.max(n);
                }
            }
        }
    }

    max_id + 1
}
