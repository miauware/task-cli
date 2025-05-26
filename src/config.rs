use std::fs;

pub fn config_dir() -> String {
    if cfg!(target_os = "windows") {
        // INFO: Use environment variable APPDATA for Windows
        std::env::var("APPDATA")
            .map(|path| format!("{}/task-cli", path))
            .unwrap_or_else(|_| String::from("C:/Users/Default/AppData/Roaming/task-cli"))
    } else {
        // INFO: Use default directory for  Linux/macOS
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/home/default"));
        format!("{}/.config/task-cli", home)
    }
}

pub fn ensure_dir_exists() {
    let dir = config_dir();
    if !std::path::Path::new(&dir).exists() {
        // INFO: Create directory if it does not exist
        fs::create_dir_all(&dir).expect("Failed to create config directory");
    }
}
