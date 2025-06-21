use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use once_cell::sync::Lazy;
use poise::{ApplicationContext, BoxFuture};
use poise::framework::{};
use songbird::input::Input;

pub struct Data {
    pub audio_map: HashMap<String, HashMap<String, PathBuf>>, // command -> clip -> path
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

type SlashHandler =
fn(ApplicationContext<'_, (), Error>) -> BoxFuture<'_, Result<(), Error>>;

pub struct SlashCommand {
    pub name: String,
    pub description: Option<String>,
    pub action: fn(poise::ApplicationContext<'_, (), Error>) -> poise::BoxFuture<'_, Result<(), Error>>,
}