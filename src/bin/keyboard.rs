use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let mut last_command: Vec<String> = Vec::new();

    loop {
        println!("\nChoose the RGB mode");
        println!("1. Static");
        println!("2. Breathing");
        println!("3. Neon");
        println!("4. Wave");
        println!("5. Shifting");
        println!("6. Zoom");
        println!("7. Re-run last command");
        println!("0. Exit");

        let choice = match read_i32("Choice: ") {
            Some(value) => value,
            None => continue,
        };

        if choice == 0 {
            break;
        }

        if choice == 7 {
            if last_command.is_empty() {
                println!("No previous command to rerun.");
            } else {
                run_facer_rgb(&last_command);
            }
            continue;
        }

        if !(1..=6).contains(&choice) {
            println!("Invalid choice.");
            continue;
        }

        let mode = choice - 1;
        let mut args = vec!["-m".to_string(), mode.to_string()];

        if mode == 0 {
            let zone = read_in_range("Zone (1-4, default 1): ", 1, 4).unwrap_or(1);
            args.extend(["-z".to_string(), zone.to_string()]);
            let (r, g, b) = read_rgb();
            args.extend([
                "-R".to_string(), r.to_string(),
                "-G".to_string(), g.to_string(),
                "-B".to_string(), b.to_string(),
            ]);
        } else {
            let speed = read_in_range("Speed (0-9, default 4): ", 0, 9).unwrap_or(4);
            let brightness = read_in_range("Brightness (0-100, default 100): ", 0, 100).unwrap_or(100);
            let direction = if mode == 3 || mode == 4 {
                read_in_range("Direction (1-2, default 1): ", 1, 2).unwrap_or(1)
            } else {
                1
            };

            args.extend(["-s".to_string(), speed.to_string()]);
            args.extend(["-b".to_string(), brightness.to_string()]);
            if mode == 3 || mode == 4 {
                args.extend(["-d".to_string(), direction.to_string()]);
            }

            if matches!(mode, 1 | 3 | 4 | 5) {
                let (r, g, b) = read_rgb();
                args.extend([
                    "-R".to_string(), r.to_string(),
                    "-G".to_string(), g.to_string(),
                    "-B".to_string(), b.to_string(),
                ]);
            }
        }

        last_command = args.clone();
        run_facer_rgb(&args);
    }
}

fn read_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    Some(input.trim().to_string())
}

fn read_i32(prompt: &str) -> Option<i32> {
    read_line(prompt)?.parse().ok()
}

fn read_in_range(prompt: &str, min: i32, max: i32) -> Option<i32> {
    loop {
        let input = read_line(prompt)?;
        if input.is_empty() {
            return None;
        }
        match input.parse::<i32>() {
            Ok(value) if (min..=max).contains(&value) => return Some(value),
            _ => println!("Enter a value between {min} and {max}."),
        }
    }
}

fn read_rgb() -> (i32, i32, i32) {
    loop {
        let Some(input) = read_line("RGB values (R G B, default 255 255 255): ") else {
            return (255, 255, 255);
        };
        if input.is_empty() {
            return (255, 255, 255);
        }

        let values: Vec<_> = input.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        if values.len() == 3 && values.iter().all(|v| (0..=255).contains(v)) {
            return (values[0], values[1], values[2]);
        }
        println!("Enter exactly three integers from 0 to 255.");
    }
}

fn run_facer_rgb(args: &[String]) {
    println!("Running facer-rgb with: {}", args.join(" "));
    let result = Command::new("facer-rgb").args(args).status();

    match result {
        Ok(status) if status.success() => {}
        Ok(status) => eprintln!("facer-rgb exited with status: {status}"),
        Err(error) => eprintln!("Could not start facer-rgb: {error}. Ensure it is installed and on PATH."),
    }

    thread::sleep(Duration::from_millis(250));
}
