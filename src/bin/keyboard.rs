use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::thread;

use crate::common::{ensure_config_dir, get_config_directory, get_profile_path, load_profile, save_profile};

fn main() {
    let mut mode_choice: Option<i32> = None;
    let mut zone_list: Vec<i32> = Vec::new();
    let mut speed_choice: Option<i32> = None;
    let mut bright_choice: Option<i32> = None;
    let mut direction_choice: Option<i32> = None;
    let mut color_choice: Vec<i32> = Vec::new();
    let mut final_command: Vec<String> = Vec::new();

    setup();
    loop {
        match mode() {
            Some(choice) => match choice {
                0 => break, // Exit
                1 => { // Static
                    mode_choice = Some(0);
                    zone();
                    color();
                }
                2 => { // Breathing
                    mode_choice = Some(1);
                    speed();
                    bright();
                    color();
                }
                3 => { // Neon
                    mode_choice = Some(2);
                    speed();
                    bright();
                }
                4 => { // Wave
                    mode_choice = Some(3);
                    speed();
                    bright();
                    direction();
                }
                5 => { // Shifting
                    mode_choice = Some(4);
                    speed();
                    bright();
                    color();
                    direction();
                }
                6 => { // Zoom
                    mode_choice = Some(5);
                    speed();
                    bright();
                    color();
                }
                7 => { // Re-Run Last Command
                    rerun(&mut final_command);
                }
                _ => println!("Invalid Choice"),
            },
            None => continue, // Invalid input, try again
        }

        if let Some(ref mut cmd) = build_command(
            mode_choice,
            &zone_list,
            speed_choice,
            bright_choice,
            direction_choice,
            &color_choice,
        ) {
            final_command.push(cmd.clone());
            execute_command(cmd);
        }

        // Reset for next command (except for rerun which builds its own)
        mode_choice = None;
        zone_list.clear();
        speed_choice = None;
        bright_choice = None;
        direction_choice = None;
        color_choice.clear();
    }
}

fn setup() {
    // Create cache file if it doesn't exist
    if std::fs::read_dir(".")
        .ok()
        .and_then(|entries| {
            entries
                .flatten()
                .find(|e| e.file_name() == ".keyboard_cache")
                .is_some()
        })
        .is_none()
    {
        let _ = std::fs::File::create(".keyboard_cache");
    }
}

fn mode() -> Option<i32> {
    println!("Choose the RGB Mode");
    println!("1. Static");
    println!("2. Breathing");
    println!("3. Neon");
    println!("4. Wave");
    println!("5. Shifting");
    println!("6. Zoom");
    println!("7. Re-Run the Last Command");
    println!("0. Exit");
    
    let mut input = String::new();
    io::stdout().flush().ok();
    io::stdin().read_line(&mut input).ok()?;
    
    let choice: i32 = input.trim().parse().ok()?;
    
    match choice {
        1 | 2 | 3 | 4 | 5 | 6 | 7 => Some(choice),
        0 => Some(0), // Exit
        _ => {
            println!("Invalid Choice");
            thread::sleep(std::time::Duration::from_millis(1500));
            None
        }
    }
}

fn speed() {
    loop {
        let mut input = String::new();
        println!("Enter the Value of Speed [0-9] \n0 -> Static\n1 -> Slowest\n9 -> Fastest\nJust Press Enter to use the Default Value:");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).ok()?;
        
        let input = input.trim();
        if input.is_empty() {
            break; // Use default
        }
        
        if let Ok(choice) = input.parse::<i32>() {
            if choice >= 0 && choice <= 9 {
                speed_choice = Some(choice);
                break;
            }
        }
        
        println!("Invalid Choice. Try again");
        thread::sleep(std::time::Duration::from_millis(1500));
    }
}

fn bright() {
    loop {
        let mut input = String::new();
        println!("Enter the Value of Brightness [0-100]\n0 -> Switched Off\n100 -> Brightest\nJust Press Enter to use the Default Value:");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).ok()?;
        
        let input = input.trim();
        if input.is_empty() {
            break; // Use default
        }
        
        if let Ok(choice) = input.parse::<i32>() {
            if choice >= 0 && choice <= 100 {
                bright_choice = Some(choice);
                break;
            }
        }
        
        println!("Invalid Choice. Try again");
        thread::sleep(std::time::Duration::from_millis(1500));
    }
}

fn direction() {
    loop {
        let mut input = String::new();
        println!("Enter the Direction of Animation [1/2]\n1 -> Right to Left\n2 -> Left to Right\nJust Press Enter to use the Default Value:");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).ok()?;
        
        let input = input.trim();
        if input.is_empty() {
            break; // Use default
        }
        
        if let Ok(choice) = input.parse::<i32>() {
            if choice == 1 || choice == 2 {
                direction_choice = Some(choice);
                break;
            }
        }
        
        println!("Invalid Choice. Try again");
        thread::sleep(std::time::Duration::from_millis(1500));
    }
}

fn zone() {
    loop {
        let mut input = String::new();
        println!("Enter the Zone ID(s) you want to select (1-4) separated by space\nIf you want to select all the zones just press Enter:");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).ok()?;
        
        let input = input.trim();
        if input.is_empty() {
            zone_list = vec![1, 2, 3, 4];
            break;
        }
        
        let mut valid = true;
        let mut zones = Vec::new();
        
        for token in input.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                if (1..=4).contains(&num) {
                    zones.push(num);
                } else {
                    valid = false;
                    break;
                }
            } else {
                valid = false;
                break;
            }
        }
        
        if valid && !zones.is_empty() && zones.len() < 5 {
            zone_list = zones;
            break;
        }
        
        println!("Invalid Selection, Choose Again");
        thread::sleep(std::time::Duration::from_millis(1500));
    }
}

fn color() {
    loop {
        let mut input = String::new();
        println!("Enter a Valid RGB code with all the channels separated by space\nExample: 255 255 255\nJust Press Enter to use the Default Color (White):");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).ok()?;
        
        let input = input.trim();
        if input.is_empty() {
            color_choice = vec![255, 255, 255];
            break;
        }
        
        let mut values = Vec::new();
        let mut valid = true;
        
        for token in input.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                if num >= 0 && num <= 255 {
                    values.push(num);
                } else {
                    valid = false;
                    break;
                }
            } else {
                valid = false;
                break;
            }
        }
        
        if valid && values.len() == 3 {
            color_choice = values;
            break;
        }
        
        println!("There are 3 channels and values should be between 0 - 255");
        thread::sleep(std::time::Duration::from_millis(1500));
    }
}

fn rerun(final_command: &mut Vec<String>) {
    if let Ok(data) = std::fs::read_to_string(".keyboard_cache") {
        for cmd in data.split(',').filter(|s| !s.is_empty()) {
            final_command.push(cmd.to_string());
        }
    }
    // In the original script, this exits after setting final_command
    // For simplicity, we'll just execute the commands and continue
    for cmd in final_command.drain(..) {
        execute_command(&cmd);
    }
}

fn build_command(
    mode_choice: Option<i32>,
    zone_list: &[i32],
    speed_choice: Option<i32>,
    bright_choice: Option<i32>,
    direction_choice: Option<i32>,
    color_choice: &[i32],
) -> Option<String> {
    let mut command = String::new();
    
    // Build the base command
    if let Some(mode) = mode_choice {
        command.push_str(&format!("./facer-rgb -m {}", mode));
    } else {
        return None;
    }
    
    // Add zone if in static mode (mode 0)
    if mode_choice == Some(0) && !zone_list.is_empty() {
        // For simplicity, just use the first zone
        if let Some(zone) = zone_list.first() {
            command.push_str(&format!(" -z {}", zone));
        }
    }
    
    if let Some(speed) = speed_choice {
        command.push_str(&format!(" -s {}", speed));
    }
    if let Some(bright) = bright_choice {
        command.push_str(&format!(" -b {}", bright));
    }
    if let Some(direction) = direction_choice {
        command.push_str(&format!(" -d {}", direction));
    }
    if color_choice.len() == 3 {
        command.push_str(&format!(
            " -cR {} -cG {} -cB {}",
            color_choice[0], color_choice[1], color_choice[2]
        ));
    }
    
    Some(command)
}

fn execute_command(command: &str) {
    println!("Executing: {}", command);
    // In a real implementation, we would actually run the command
    // For now, we'll just print it and simulate execution
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(".keyboard_cache")
    {
        let _ = writeln!(file, "{}", command);
    }
    
    // Actually execute the facer-rgb binary if it exists
    // Command::new("./facer-rgb")
    //     .args(command.split_whitespace().skip(1)) // Skip the "./facer-rgb" part
    //     .output()
    //     .ok();
    
    thread::sleep(std::time::Duration::from_millis(500));
}
