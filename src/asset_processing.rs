use std::collections::HashMap;
use std::{env, fs};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use songbird::input::Input;
use crate::enums::AssetClass;
use crate::types::{CommandInfo, Context, Data, Error, GreetingCommand};
use poise::{serenity_prelude as serenity};
use poise::serenity_prelude::CommandData;
use rand::seq::IndexedRandom;
use crate::extensions::{HashSetExt};
use walkdir::WalkDir;

static FRANTA_CUS: &str = "franta\\cus.mp3";
static FRANTA_SERVUS: &str = "franta\\servus.mp3";
static FRANTA_ZDRAVIMTE: &str = "franta\\zdravimte.mp3";

pub async fn franta_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<String> {
    FRANTA_ASSETS.get_fitting_keys(partial)
}

static FRANTA_ASSETS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("buzerante", "franta\\buzerante.mp3"),
        ("cus", FRANTA_CUS),
        ("cernakundo", "franta\\cernakundo.mp3"),
        ("gadze", "franta\\gadze.mp3"),
        ("jitrnice", "franta\\buzerante.mp3"),
        ("kaizer", "franta\\kaizer.mp3"),
        ("koniny", "franta\\koniny.mp3"),
        ("libusko", "franta\\libusko.mp3"),
        ("mekac", "franta\\mekac.mp3"),
        ("nejebe", "franta\\nejebe.mp3"),
        ("nesundas", "franta\\nesundas.mp3"),
        ("servus", FRANTA_SERVUS),
        ("zdravimte", FRANTA_ZDRAVIMTE),
    ])
});

static DUFKA_RASTAFA: &str = "dufka\\rastafa.mp3";

pub async fn dufka_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<String> {
    DUFKA_ASSETS.get_fitting_keys(partial)
}

static DUFKA_ASSETS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("rastafa", DUFKA_RASTAFA),
    ])
});

pub async fn cojetypico_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<String> {
    COJETYPICO_ASSETS.get_fitting_keys(partial)
}

static COJETYPICO_ASSETS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("cojetypico", "cojetypico\\cojetypico.mp3"),
        ("rorodina", "cojetypico\\rorodina.mp3"),
    ])
});

pub async fn misc_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<String> {
    MISC_ASSETS.get_fitting_keys(partial)
}

static MISC_PANEMACO: &str = "misc\\panemaco.mp3";
static MISC_STAVO: &str = "misc\\stavo.mp3";

static MISC_ASSETS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("dalsiotazka", "misc\\dalsiotazka.mp3"),
        ("dobrabyla", "misc\\dobrabyla.mp3"),
        ("stavo", MISC_STAVO),
        ("aco", "misc\\aco.mp3"),
        ("panemaco", MISC_PANEMACO),
    ]) 
});

pub async fn zesrane_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<String> {
    ZESRANE_ASSETS.get_fitting_keys(partial)
}

static ZESRANE_ASSETS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("jatojebu", "zesrane\\jatojebu.mp3"),
        ("taktone", "misc\\taktone.mp3"),
        ("zesrane", "misc\\zesrane.mp3"),
    ])
});

pub async fn dota_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<String> {
    println!("command name: {}", _ctx.command().name);
    DOTA_ASSETS.get_fitting_keys(partial)
}

static DOTA_ASSETS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("easiestmoney", "dota\\easiestmoney.mp3"),
        ("echoslammajamma", "dota\\echoslammajamma.mp3"),
        ("lakad", "dota\\lakad.mp3"),
        ("nochill", "dota\\nochill.mp3"),
        ("ojojoj", "dota\\ojojoj.mp3"),
    ])
});

static RANDOM_GREETINGS: &'static [&'static str] = &[
    FRANTA_CUS,
    FRANTA_SERVUS,
    FRANTA_ZDRAVIMTE
];

static USER_GREETINGS: Lazy<HashMap<u64, Vec<&'static str>>> = Lazy::new(|| {
    HashMap::from([
        (419115472471064576, vec![DUFKA_RASTAFA, MISC_PANEMACO]), // maca
        (419115479551180820, vec![MISC_STAVO]), // verca
        (450691668740669450, vec!["aco"]), // ja
    ])
});


/// Chooses a greeting of an incoming user. If the user has no pre-defined greetings, chooses a random one.
pub fn choose_greetings(user_id: &serenity::UserId, data: &Data) -> Result<Input, Error> {
    let Some(greetings_fallback) = data.config.greetings.get("_fallback") else {
        return Err("No fallback greetings defined.".into());
    };
    
    let choices = data.config.greetings
        .get(user_id.to_string().as_str())
        .unwrap_or(greetings_fallback);
    let mut rng = rand::rng();
    let choice = choices.choose(&mut rng).unwrap();
    
    let Some(command) = data.audio_map.get(choice.command.as_str()) else {
        return Err(format!("No such asset - {}", choice.command).into());
    };
    let path: &PathBuf = match command {
        CommandInfo::Path(path) => {
            Ok::<&PathBuf, Error>(path)
        },
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
    get_input(path)
}

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

fn get_input(path: &PathBuf) -> Result<Input, Error> {
    println!("Current dir: {:?}", env::current_dir()?);
    match fs::read(path.clone()) {
        Ok(data) => Ok(songbird::input::Input::from(data)),
        Err(_) => Err(format!("File {} not found.", path.to_string_lossy()).into())
    }
}

struct CommandOption {
    option: String,

}

pub fn visit_dirs(dir: &Path, asset_map: &mut HashMap<String, Vec<String>>) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, asset_map)?; // Recurse
            } else if path.is_file() {
                if let (Some(parent), Some(stem)) = (
                    path.parent().and_then(|p| p.file_name()),
                    path.file_stem()
                ) {
                    let command = parent.to_string_lossy().to_string();
                    let option = stem.to_string_lossy().to_string();

                    asset_map
                        .entry(command)
                        .or_insert_with(Vec::new)
                        .push(option);
                }
            }
        }
    }
    Ok(())
}
