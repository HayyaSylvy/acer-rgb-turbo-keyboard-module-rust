use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use dirs::home_dir;

pub fn get_config_directory() -> PathBuf {
    let home = home_dir().expect("Could not find home directory");
    home.join(".config/predator/saved profiles")
}

pub fn ensure_config_dir() {
    let config_dir = get_config_directory();
    let _ = fs::create_dir_all(&config_dir);
}

pub fn get_profile_path(name: &str) -> PathBuf {
    get_config_directory().join(format!("{}.json", name))
}

pub fn save_profile(name: &str, args: &HashMap<String, i32>) -> io::Result<()> {
    ensure_config_dir();
    let path = get_profile_path(name);
    let mut file = File::create(path)?;
    
    // Convert args to JSON-like format for simplicity
    let mut json_str = String::new();
    json_str.push('{');
    let mut first = true;
    for (key, value) in args {
        if !first {
            json_str.push_str(", ");
        }
        first = false;
        json_str.push_str(format!("\"{}\": {}", key, value).as_str());
    }
    json_str.push('}');
    
    file.write_all(json_str.as_bytes())?;
    Ok(())
}

pub fn load_profile(name: &str) -> Option<HashMap<String, i32>> {
    let path = get_profile_path(name);
    let content = fs::read_to_string(path).ok()?;
    
    // Simple JSON parser for our specific format
    let mut args = HashMap::new();
    let content = content.trim();
    if content.len() < 2 || !content.starts_with('{') || !content.ends_with('}') {
        return None;
    }
    
    let inner = &content[1..content.len()-1];
    if inner.is_empty() {
        return Some(args);
    }
    
    for pair in inner.split(", ") {
        let mut parts = pair.splitn(2, ": ");
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let key = key.trim_matches('"');
            if let Ok(val) = value.parse::<i32>() {
                args.insert(key.to_string(), val);
            }
        }
    }
    
    Some(args)
}
