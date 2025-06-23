use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

pub struct BotData {
    pub audio_map: HashMap<String, CommandInfo>, // command -> clip -> path
    pub config: Config 
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub greetings: HashMap<String,Vec<GreetingCommand>>
}

/// Maps a certain asset file to a greeting
#[derive(Debug, Deserialize)]
pub struct GreetingCommand {
    pub command: String,
    pub option: Option<String>,
    pub _label: Option<String>,
}

pub enum CommandInfo {
    Options(HashMap<String, PathBuf>),
    Path(PathBuf),
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, BotData, Error>;
