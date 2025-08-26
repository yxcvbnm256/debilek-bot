use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use songbird::input::Input;
use crate::types::{CommandInfo, BotData, Error};
use poise::{serenity_prelude as serenity};
use rand::seq::IndexedRandom;
use walkdir::WalkDir;


/// Chooses a greeting of an incoming user. If the user has no pre-defined greetings, chooses a random one.
pub fn choose_greetings(user_id: &serenity::UserId, data: &BotData) -> Result<Input, Error> {
    // Fallback greetings should be defined when no user-specific greeting is present.
    let Some(greetings_fallback) = data.config.greetings.get("_fallback") else {
        return Err("No fallback greetings defined.".into());
    };
    
    let user_greetings = data.config.greetings
        .get(user_id.to_string().as_str())
        .unwrap_or(greetings_fallback);
    
    let mut rng = rand::rng();
    let choice = user_greetings.choose(&mut rng).unwrap();
    
    let Some(command) = data.audio_map.get(choice.command.as_str()) else {
        return Err(format!("No such asset - {}", choice.command).into());
    };
    let path: &PathBuf = match command {
        // asset of a "sub" command with option
        CommandInfo::Path(path) => {
            Ok::<&PathBuf, Error>(path)
        },
        // asset, which is directly a command
        CommandInfo::Options(options) => {
            let Some(option) = &choice.option else {
                return Err("No option specified.".into())
            };
            
            let Some(path) = options.get(option) else {
                return Err(format!("No such option - {}", option).into())
            };
            Ok(path)
        }
    }?;
    get_songbird_input(path)
}

/// Discovers the audio structure of the specified folder. Supported structure:
/// root audio files -> its own command.
/// Subfolder (only level 1) -> a command.
/// Audio files in subfolder -> a command option.
/// Ignores assets, which are in the ignore_commands config list
pub fn discover_audio_structure(base: &Path) -> HashMap<String, CommandInfo> {
    let mut map: HashMap<String, CommandInfo> = HashMap::new();

    for entry in WalkDir::new(base).min_depth(0).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().is_some() {
            if let Some(folder) = path.parent().and_then(|p| p.strip_prefix(base).ok()) {
                let folder_name = folder.to_string_lossy().to_string();
                let file_stem = path.file_stem().unwrap().to_string_lossy().to_string();
                if folder_name.is_empty() {
                    map
                        .entry(file_stem.clone())
                        .insert_entry(CommandInfo::Path(path.to_path_buf()));
                } else {
                    match map.entry(folder_name.clone()).or_default() {
                        CommandInfo::Path(_) => {
                            println!("Folder {:?} is clashing with {:?}. Ignoring...", folder_name, file_stem);
                        },
                        CommandInfo::Options(options) => {
                            options.insert(file_stem.clone(), path.to_path_buf());
                        }
                    }
                }                
            }
        }
    }

    map
}

/// Reads the audio file from the specified path.
fn get_songbird_input(path: &PathBuf) -> Result<Input, Error> {
    match fs::read(path.clone()) {
        Ok(data) => Ok(songbird::input::Input::from(data)),
        Err(_) => Err(format!("File {} not found.", path.to_string_lossy()).into())
    }
}
