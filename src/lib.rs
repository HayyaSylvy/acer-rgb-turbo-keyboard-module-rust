
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use dirs::home_dir;

pub mod common {
    use super::*;

    pub fn get_config_directory() -> PathBuf {
        home_dir()
            .expect("Could not find home directory")
            .join(".config/predator/saved profiles")
    }

    pub fn ensure_config_dir() {
        let _ = fs::create_dir_all(get_config_directory());
    }

    pub fn get_profile_path(name: &str) -> PathBuf {
        // Prevent profile names from escaping the profile directory.
        let safe_name = name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>();

        get_config_directory().join(format!("{safe_name}.json"))
    }

    pub fn save_profile(
        name: &str,
        args: &HashMap<String, i32>,
    ) -> io::Result<()> {
        ensure_config_dir();

        let path = get_profile_path(name);
        let mut file = File::create(path)?;

        // Keep the existing simple profile format.
        let mut entries: Vec<_> = args.iter().collect();
        entries.sort_by_key(|(key, _)| *key);

        write!(file, "{{")?;

        for (index, (key, value)) in entries.iter().enumerate() {
            if index != 0 {
                write!(file, ", ")?;
            }

            write!(file, "\"{}\": {}", key, value)?;
        }

        writeln!(file, "}}")?;
        Ok(())
    }

    pub fn load_profile(name: &str) -> Option<HashMap<String, i32>> {
        let content = fs::read_to_string(get_profile_path(name)).ok()?;

        // Parse the simple format written by save_profile.
        let content = content.trim();

        if !content.starts_with('{') || !content.ends_with('}') {
            return None;
        }

        let inner = &content[1..content.len() - 1];
        let mut args = HashMap::new();

        if inner.trim().is_empty() {
            return Some(args);
        }

        for pair in inner.split(", ") {
            let (key, value) = pair.split_once(": ")?;
            let key = key.trim().trim_matches('"');
            let value = value.trim().parse::<i32>().ok()?;

            args.insert(key.to_string(), value);
        }

        Some(args)
    }
}
