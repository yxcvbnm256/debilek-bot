use std::collections::HashMap;
use std::{env, fs};
use once_cell::sync::Lazy;
use songbird::input::Input;
use crate::enums::AssetClass;
use crate::types::{Context, Error};
use poise::{serenity_prelude as serenity};
use rand::seq::IndexedRandom;
use crate::extensions::HashSetExt;

pub static FRANTA_CUS: &str = "franta\\cus.mp3";
pub static FRANTA_SERVUS: &str = "franta\\servus.mp3";
pub static FRANTA_ZDRAVIMTE: &str = "franta\\zdravimte.mp3";

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

pub static DUFKA_RASTAFA: &str = "dufka\\rastafa.mp3";

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

static MISC_ASSETS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("dalsiotazka", "misc\\dalsiotazka.mp3"),
        ("dobrabyla", "misc\\dobrabyla.mp3"),
        ("stavo", "misc\\stavo.mp3"),
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

static RANDOM_GREETINGS: &'static [&'static str] = &[
    FRANTA_CUS,
    FRANTA_SERVUS,
    FRANTA_ZDRAVIMTE
];

static USER_GREETINGS: Lazy<HashMap<u64, Vec<&'static str>>> = Lazy::new(|| {
    HashMap::from([
        (419115472471064576, vec![DUFKA_RASTAFA]), // maca
        //(serenity::UserId::from(450691668740669450), vec!["./assets/franta/servus.mp3"]), // ja
    ])
});

/// Gets an asset file to play.
pub fn get_asset_file(asset_class: AssetClass, asset_name: &str) -> Result<Input, Error> {
    let asset = match asset_class {
        AssetClass::Franta => FRANTA_ASSETS.get(asset_name),
        AssetClass::Dufka => DUFKA_ASSETS.get(asset_name),
        AssetClass::Cojetypico => COJETYPICO_ASSETS.get(asset_name),
        AssetClass::Misc => MISC_ASSETS.get(asset_name),   
        AssetClass::ZesraneHajzle => ZESRANE_ASSETS.get(asset_name),
    };
    
    let Some(asset) = asset else { return Err("Tohle tu nemám, debílku.".into()) };
    get_input(asset)
}


/// Chooses a greeting of an incoming user. If the user has no pre-defined greetings, chooses a random one.
pub fn choose_greetings(user_id: &serenity::UserId) -> Result<Input, Error> {
    let mut rng = rand::rng();
    let used_id_int = user_id.get();
    match USER_GREETINGS.get(&used_id_int) {
        Some(values) => {
            let asset = values.choose(&mut rng).unwrap();
            get_input(asset)
        },
        None => {
            let asset = RANDOM_GREETINGS.choose(&mut rng).unwrap();
            get_input(asset)
        }
    }
}

fn get_input(asset_name: &str) -> Result<Input, Error> {
    let mut path = env::current_dir()?;
    path.push("assets");
    path.push(asset_name);
    let data = fs::read(path)?;
    Ok(songbird::input::Input::from(data))
}
