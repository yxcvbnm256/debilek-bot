use std::collections::HashMap;
use std::{env, fs};
use std::path::PathBuf;
use anyhow::anyhow;
use poise::{ApplicationContext, BoxFuture, Command, CommandParameterChoice, SlashArgError};
use poise::serenity_prelude::{CommandOptionType, CreateAutocompleteResponse, ResolvedOption, ResolvedValue};
use poise::serenity_prelude::json::to_string;
use reqwest::Client;
use songbird::input::Input;
use songbird::serenity;
use crate::asset_processing::get_asset_file;
use crate::asset_processing::franta_autocomplete;
use crate::asset_processing::dufka_autocomplete;
use crate::asset_processing::zesrane_autocomplete;
use crate::asset_processing::misc_autocomplete;
use crate::asset_processing::cojetypico_autocomplete;
use crate::asset_processing::dota_autocomplete;
use crate::enums::AssetClass;
use crate::types::{Context, Data, Error};
use crate::voice::play;

/*/// Plays stupid voice stuff
#[poise::command(slash_command, prefix_command)]
pub async fn sound(
    ctx: Context<'_>,
    #[description = "What to be played"] text: String,
    #[description = "In which language (en, pl, ...)"] lang: Option<String>,
) -> Result<(), Error> {

    let client = Client::new();

    let key = env::var("TTS_KEY")
        .expect("TTS key not set.");

    ctx.defer().await?;
    
    let response = client
        .get("https://texttospeech.responsivevoice.org/v1/text:synthesize")
        .query(&[
            ("text", text.clone()),
            ("lang", lang.unwrap_or_else(|| "cs".into())),
            ("voice", "female".into()),
            ("engine", "g1".into()),
            ("key",key),
        ])
        .send()
        .await?
        .error_for_status();

    match response {
        Ok(r) => {
            let src = songbird::input::Input::from(r.bytes().await?);
            execute_voice_command(ctx, text, src).await
        },
        Err(e) => {
            println!("{:?}", e.status().unwrap().as_str());
            Err("Text to speech request failed, sorry.".into())
        }
    }
}

/// Plays cojetypíčhoo
#[poise::command(slash_command, prefix_command)]
pub async fn cojetypico(
    ctx: Context<'_>,
    #[description = "What to be played"]
    #[autocomplete = "cojetypico_autocomplete"]
    option: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let asset = get_asset_file(AssetClass::Cojetypico, option.as_str())?;
    execute_voice_command(ctx, option, asset).await
}

/// Plays franta
#[poise::command(slash_command, prefix_command)]
pub async fn franta(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"]
    #[autocomplete = "franta_autocomplete"]
    option: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let asset = get_asset_file(AssetClass::Franta, option.as_str())?;
    execute_voice_command(ctx, option, asset).await
}

/// Plays zesrané hajzle
#[poise::command(slash_command, prefix_command)]
pub async fn zesrane(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"]
    #[autocomplete = "zesrane_autocomplete"]
    option: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let asset = get_asset_file(AssetClass::ZesraneHajzle, option.as_str())?;
    execute_voice_command(ctx, option, asset).await
}

/// Plays viktor
#[poise::command(slash_command, prefix_command)]
pub async fn dufka(
    ctx: Context<'_>,
    #[description = "What to be played"]
    #[autocomplete = "dufka_autocomplete"]
    option: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let asset = get_asset_file(AssetClass::Dufka, option.as_str())?;
    execute_voice_command(ctx, option, asset).await
}

/// Plays whatever
#[poise::command(slash_command, prefix_command)]
pub async fn misc(
    ctx: Context<'_>,
    #[description = "What to be played"]
    #[autocomplete = "misc_autocomplete"]
    option: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let asset = get_asset_file(AssetClass::Misc, option.as_str())?;
    execute_voice_command(ctx, option, asset).await
}
*/
/// Plays dota bullshit
#[poise::command(slash_command)]
pub async fn dota(
    ctx: Context<'_>,
    #[description = "What to be played"]
    #[autocomplete = "dota_autocomplete"]
    option: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let asset = get_asset_file(AssetClass::Dota, option.as_str())?;
    execute_voice_command(ctx, option, asset).await
}

/// TODO finish this
pub async fn generic_voice_command(ctx: Context<'_>, text: String, input: songbird::input::Input) -> Result<(), Error> {
    play(ctx, input).await.or_else(|e| Err(e))?;
    ctx.send(poise::CreateReply::default()
        .content(text)
        .ephemeral(true)
    ).await?;
    Ok(())
}

async fn execute_voice_command(ctx: Context<'_>, text: String, input: songbird::input::Input) -> Result<(), Error> {
    play(ctx, input).await.or_else(|e| Err(e))?;
    ctx.send(poise::CreateReply::default()
        .content(text)
        .ephemeral(true)
    ).await?;
    Ok(())
}

pub type CommandFuture<'a> = std::pin::Pin<
    Box<dyn Future<Output = Result<(), poise::FrameworkError<'a, Data, Error>>> + Send + 'a>
>;

pub fn make_audio_handler(
    command_name: String,
) -> for<'a> fn(poise::ApplicationContext<'a, Data, Error>) -> CommandFuture<'a> {
    fn handler<'a>(
        ctx: poise::ApplicationContext<'a, Data, Error>,
    ) -> CommandFuture<'a> {
        Box::pin(async move {
            ctx.defer().await;
            println!("Audio command called");
            let command = &ctx.command().name;
            let Some(clip) = get_string_from_resolved(&ctx.args) else {
                return Ok(());
            };
            //let clip = ctx.get_string("clip")?.to_string();

            let all_clips = &ctx.data().audio_map;
            if let Some(clip_map) = all_clips.get(command) {
                if let Some(path) = clip_map.get(clip) {
                    ctx.say(format!("Would play: {}", path.display())).await;
                    match fs::read(path.clone()) {
                        Ok(data) => {
                            play(Context::from(ctx), songbird::input::Input::from(data)).await;
                            return Ok(())
                        },
                        Err(_) => return Ok(())
                    }
                } else {
                    ctx.say("Clip not found").await;
                }
            } else {
                ctx.say("Category not found").await;
            }

            return Ok(())
        })
    }

    handler
}

fn get_string_from_resolved<'a>(args: &&[ResolvedOption<'a>]) -> Option<&'a str> {
    let Some(first) = args.first() else { return None };
    match first.value {
        ResolvedValue::String(s) => Some(s),
        _ => None,
    }
}

pub fn make_audio_command(
    command_name: String,
    clips: HashMap<String, PathBuf>,
) -> Command<Data, Error> {
    let choices: Vec<String> = clips.keys().cloned().collect();

    poise::Command {
        identifying_name: command_name.clone(),
        name: command_name.clone(),
        qualified_name: command_name.clone(),
        source_code_name: command_name.clone(),
        description: Some("Play a clip from this category".into()),
        ephemeral: true,
        parameters: vec![poise::CommandParameter {
            name: "clip".into(),
            name_localizations: Default::default(),
            description: Some("Choose a clip".into()),
            /*choices: choices.iter().map(|c| CommandParameterChoice {
                name: c.to_string(),
                localizations: Default::default(),
                __non_exhaustive: (),
            } ).collect(),*/
            choices: vec!(),
            autocomplete_callback: Some(my_autocomplete),
            required: true,
            channel_types: None,
            type_setter: Some(|option| option.kind(CommandOptionType::String)),
            //autocomplete_callback: None,
            description_localizations: Default::default(),
            __non_exhaustive: (),
        }],

        slash_action: Some(make_audio_handler(command_name)),
        ..Default::default()
    }
}

fn my_autocomplete<'a>(
    ctx: ApplicationContext<'a, Data, Error>,
    input: &'a str,
) -> BoxFuture<'a, Result<CreateAutocompleteResponse, SlashArgError>> {
    Box::pin(async move {
        println!("Autocomplete called");
        let mut response = CreateAutocompleteResponse::default();
        Ok(response)
    })
}