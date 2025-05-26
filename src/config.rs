pub static EMOJIS : [char; 2] =['',''] ;

// INFO: Return the config path for diferents SO
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

