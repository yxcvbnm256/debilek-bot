use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use once_cell::sync::Lazy;
use poise::{ApplicationContext, BoxFuture};
use poise::framework::{};
use songbird::input::Input;

pub struct Data {
    pub audio_map: HashMap<String, CommandInfo>, // command -> clip -> path
}

pub enum CommandInfo {
    Options(HashMap<String, PathBuf>),
    Path(PathBuf),
}

impl Default for CommandInfo {
    fn default() -> Self {
        CommandInfo::Options(HashMap::new())  // or MyData::Path(PathBuf::new())
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;
