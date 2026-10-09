use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use clap::{ArgAction, ArgMatches, Command};
use crate::common::{ensure_config_dir, get_config_directory, get_profile_path, load_profile, save_profile};

const PAYLOAD_SIZE: usize = 16;
const CHARACTER_DEVICE: &str = "/dev/acer-gkbbl-0";

const PAYLOAD_SIZE_STATIC_MODE: usize = 4;
const CHARACTER_DEVICE_STATIC: &str = "/dev/acer-gkbbl-static-0";

fn main() {
    let matches = Command::new("facer-rgb")
        .about("Interacts with experimental Acer-wmi kernel module")
        .arg(
            clap::Arg::new("mode")
                .short('m')
                .long("mode")
                .value_parser(clap::value_parser!(i32))
                .default_value("3")
                .help("Effect modes:\n\
                    0 -> Static [Accepts ZoneID[1,2,3,4] + RGB Color]\n\
                    1 -> Breath [Accepts RGB color]\n\
                    2 -> Neon\n\
                    3 -> Wave\n\
                    4 -> Shifting [Accepts RGB color]\n\
                    5 -> Zoom [Accepts RGB color]")
        )
        .arg(
            clap::Arg::new("zone")
                .short('z')
                .long("zone")
                .value_parser(clap::value_parser!(i32))
                .default_value("1")
                .help("Zone ID(Only in static mode):\n\
                    Possible values: 1,2,3,4")
        )
        .arg(
            clap::Arg::new("speed")
                .short('s')
                .long("speed")
                .value_parser(clap::value_parser!(i32))
                .default_value("4")
                .help("Animation Speed:\n\
                    0 -> No animation speed (static)\n\
                    1 -> Slowest animation speed\n\
                    9 -> Fastest animation speed\n\
                    You can use values between 1-9 to adjust the speed or increase speed even more than 255, but keep in mind\n\
                    that values higher than 9 were not used in the official PredatorSense application.")
        )
        .arg(
            clap::Arg::new("brightness")
                .short('b')
                .long("brightness")
                .value_parser(clap::value_parser!(i32))
                .default_value("100")
                .help("Keyboard backlight Brightness:\n\
                    0   -> No backlight (turned off)\n\
                    100 -> Maximum backlight brightness")
        )
        .arg(
            clap::Arg::new("direction")
                .short('d')
                .long("direction")
                .value_parser(clap::value_parser!(i32))
                .default_value("1")
                .help("Animation direction:\n\
                    1   -> Right to Left\n\
                    2   -> Left to Right")
        )
        .arg(
            clap::Arg::new("red")
                .short('cR')
                .long("red")
                .value_parser(clap::value_parser!(i32))
                .default_value("50")
                .help("Some modes require specific [R]GB color\n\
                    0   -> Minimum red range\n\
                    255 -> Maximum red range")
        )
        .arg(
            clap::Arg::new("green")
                .short('cG')
                .long("green")
                .value_parser(clap::value_parser!(i32))
                .default_value("255")
                .help("Some modes require specific R[G]B color\n\
                    0   -> Minimum green range\n\
                    255 -> Maximum green range")
        )
        .arg(
            clap::Arg::new("blue")
                .short('cB')
                .long("blue")
                .value_parser(clap::value_parser!(i32))
                .default_value("50")
                .help("Some modes require specific RG[B] color\n\
                    0   -> Minimum blue range\n\
                    255 -> Maximum blue range")
        )
        .arg(
            clap::Arg::new("save")
                .long("save")
                .action(ArgAction::Set)
                .help("Add as last argument to save to a profile")
        )
        .arg(
            clap::Arg::new("load")
                .long("load")
                .action(ArgAction::Set)
                .help("Loads the profile if it exists")
        )
        .arg(
            clap::Arg::new("list")
                .long("list")
                .action(ArgAction::SetTrue)
                .help("Lists all the saved profiles in config directory")
        )
        .get_matches();

    handle_commands(matches);
}

fn handle_commands(matches: ArgMatches) {
    if matches.get_flag("list") {
        list_profiles();
        return;
    }

    let mut args = HashMap::new();
    args.insert("mode", matches.get_one::<i32>("mode").copied().unwrap_or(3));
    args.insert("zone", matches.get_one::<i32>("zone").copied().unwrap_or(1));
    args.insert("speed", matches.get_one::<i32>("speed").copied().unwrap_or(4));
    args.insert("brightness", matches.get_one::<i32>("brightness").copied().unwrap_or(100));
    args.insert("direction", matches.get_one::<i32>("direction").copied().unwrap_or(1));
    args.insert("red", matches.get_one::<i32>("red").copied().unwrap_or(50));
    args.insert("green", matches.get_one::<i32>("green").copied().unwrap_or(255));
    args.insert("blue", matches.get_one::<i32>("blue").copied().unwrap_or(50));

    if let Some(load_name) = matches.get_one::<String>("load") {
        if let Some(loaded_args) = load_profile(load_name) {
            args.extend(loaded_args);
        }
    }

    if let Some(save_name) = matches.get_one::<String>("save") {
        // Remove save and load keys before saving
        args.remove("save");
        args.remove("load");
        if let Err(e) = save_profile(save_name, &args) {
            eprintln!("Error saving profile: {}", e);
        }
        // Re-add for potential use in the command execution
        args.insert("save", 0); // Dummy value
        args.insert("load", 0); // Dummy value
    }

    let mode = *args.get(&"mode").unwrap_or(&3);
    let zone = *args.get(&"zone").unwrap_or(&1);
    let speed = *args.get(&"speed").unwrap_or(&4);
    let brightness = *args.get(&"brightness").unwrap_or(&100);
    let direction = *args.get(&"direction").unwrap_or(&1);
    let red = *args.get(&"red").unwrap_or(&50);
    let green = *args.get(&"green").unwrap_or(&255);
    let blue = *args.get(&"blue").unwrap_or(&50);

    if mode == 0 {
        // Static coloring mode
        if zone < 1 || zone > 4 {
            eprintln!("Invalid Zone ID entered! Possible values are: 1, 2, 3, 4 from left to right");
            return;
        }
        
        let mut payload = [0u8; PAYLOAD_SIZE_STATIC_MODE];
        payload[0] = 1u8 << (zone - 1);
        payload[1] = red as u8;
        payload[2] = green as u8;
        payload[3] = blue as u8;
        
        if let Err(e) = write_to_device(CHARACTER_DEVICE_STATIC, &payload) {
            eprintln!("Error writing to static device: {}", e);
        }

        // Tell WMI To use STATIC coloring
        // Dynamic coloring mode
        let mut payload = [0u8; PAYLOAD_SIZE];
        payload[2] = brightness as u8;
        payload[9] = 1;
        
        if let Err(e) = write_to_device(CHARACTER_DEVICE, &payload) {
            eprintln!("Error writing to device: {}", e);
        }
    } else {
        // Dynamic coloring mode
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

        if let Err(e) = write_to_device(CHARACTER_DEVICE, &payload) {
            eprintln!("Error writing to device: {}", e);
        }
    }
}

fn write_to_device(path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

fn list_profiles() {
    ensure_config_dir();
    let config_dir = get_config_directory();
    
    println!("Saved profiles:");
    if let Ok(entries) = fs::read_dir(config_dir) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".json") {
                    let profile_name = &file_name[..file_name.len()-5];
                    println!("\t{}", profile_name);
                }
            }
        }
    }
}
