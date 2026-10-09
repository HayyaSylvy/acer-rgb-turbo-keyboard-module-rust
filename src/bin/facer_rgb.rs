use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};

use acer_predator_turbo_rgb::common::{
    ensure_config_dir, get_config_directory, load_profile, save_profile,
};
use clap::{Arg, ArgAction, ArgMatches, Command};

const PAYLOAD_SIZE: usize = 16;
const CHARACTER_DEVICE: &str = "/dev/acer-gkbbl-0";
const PAYLOAD_SIZE_STATIC_MODE: usize = 4;
const CHARACTER_DEVICE_STATIC: &str = "/dev/acer-gkbbl-static-0";

fn main() {
    let matches = Command::new("facer-rgb")
        .about("Interacts with the Acer RGB keyboard device")
        .arg(Arg::new("mode").short('m').long("mode").value_parser(clap::value_parser!(i32)).default_value("3"))
        .arg(Arg::new("zone").short('z').long("zone").value_parser(clap::value_parser!(i32)).default_value("1"))
        .arg(Arg::new("speed").short('s').long("speed").value_parser(clap::value_parser!(i32)).default_value("4"))
        .arg(Arg::new("brightness").short('b').long("brightness").value_parser(clap::value_parser!(i32)).default_value("100"))
        .arg(Arg::new("direction").short('d').long("direction").value_parser(clap::value_parser!(i32)).default_value("1"))
        .arg(Arg::new("red").short('R').long("red").value_parser(clap::value_parser!(i32)).default_value("50"))
        .arg(Arg::new("green").short('G').long("green").value_parser(clap::value_parser!(i32)).default_value("255"))
        .arg(Arg::new("blue").short('B').long("blue").value_parser(clap::value_parser!(i32)).default_value("50"))
        .arg(Arg::new("save").long("save").action(ArgAction::Set))
        .arg(Arg::new("load").long("load").action(ArgAction::Set))
        .arg(Arg::new("list").long("list").action(ArgAction::SetTrue))
        .get_matches();

    handle_commands(matches);
}

fn handle_commands(matches: ArgMatches) {
    if matches.get_flag("list") {
        list_profiles();
        return;
    }

    let mut args: HashMap<String, i32> = HashMap::from([
        ("mode".to_string(), *matches.get_one::<i32>("mode").unwrap_or(&3)),
        ("zone".to_string(), *matches.get_one::<i32>("zone").unwrap_or(&1)),
        ("speed".to_string(), *matches.get_one::<i32>("speed").unwrap_or(&4)),
        ("brightness".to_string(), *matches.get_one::<i32>("brightness").unwrap_or(&100)),
        ("direction".to_string(), *matches.get_one::<i32>("direction").unwrap_or(&1)),
        ("red".to_string(), *matches.get_one::<i32>("red").unwrap_or(&50)),
        ("green".to_string(), *matches.get_one::<i32>("green").unwrap_or(&255)),
        ("blue".to_string(), *matches.get_one::<i32>("blue").unwrap_or(&50)),
    ]);

    if let Some(name) = matches.get_one::<String>("load") {
        match load_profile(name) {
            Some(profile) => args.extend(profile),
            None => eprintln!("Profile not found or invalid: {name}"),
        }
    }

    if let Some(name) = matches.get_one::<String>("save") {
        if let Err(error) = save_profile(name, &args) {
            eprintln!("Error saving profile: {error}");
        }
    }

    let get = |key: &str, default: i32| *args.get(key).unwrap_or(&default);
    let mode = get("mode", 3);
    let zone = get("zone", 1);
    let speed = get("speed", 4);
    let brightness = get("brightness", 100);
    let direction = get("direction", 1);
    let red = get("red", 50);
    let green = get("green", 255);
    let blue = get("blue", 50);

    if !(0..=5).contains(&mode) {
        eprintln!("Invalid mode; expected a value from 0 to 5.");
        return;
    }
    if !(0..=255).contains(&brightness)
        || !(0..=255).contains(&red)
        || !(0..=255).contains(&green)
        || !(0..=255).contains(&blue)
    {
        eprintln!("Brightness and RGB values must be between 0 and 255.");
        return;
    }

    if mode == 0 {
        if !(1..=4).contains(&zone) {
            eprintln!("Invalid zone; expected 1, 2, 3, or 4.");
            return;
        }
        let mut payload = [0u8; PAYLOAD_SIZE_STATIC_MODE];
        payload[0] = 1u8 << (zone - 1);
        payload[1] = red as u8;
        payload[2] = green as u8;
        payload[3] = blue as u8;

        if let Err(error) = write_to_device(CHARACTER_DEVICE_STATIC, &payload) {
            eprintln!("Error writing static color: {error}");
            return;
        }

        let mut payload = [0u8; PAYLOAD_SIZE];
        payload[2] = brightness as u8;
        payload[9] = 1;
        if let Err(error) = write_to_device(CHARACTER_DEVICE, &payload) {
            eprintln!("Error enabling static mode: {error}");
        }
    } else {
        let mut payload = [0u8; PAYLOAD_SIZE];
        payload[0] = mode as u8;
        payload[1] = speed as u8;
        payload[2] = brightness as u8;
        payload[3] = if mode == 3 { 8 } else { 0 };
        payload[4] = direction as u8;
        payload[5] = red as u8;
        payload[6] = green as u8;
        payload[7] = blue as u8;
        payload[9] = 1;

        if let Err(error) = write_to_device(CHARACTER_DEVICE, &payload) {
            eprintln!("Error writing to device: {error}");
        }
    }
}

fn write_to_device(path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::options().write(true).open(path)?;
    file.write_all(data)
}

fn list_profiles() {
    ensure_config_dir();
    println!("Saved profiles:");
    match fs::read_dir(get_config_directory()) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if let Some(profile) = name.strip_suffix(".json") {
                        println!("\t{profile}");
                    }
                }
            }
        }
        Err(error) => eprintln!("Could not list profiles: {error}"),
    }
}
