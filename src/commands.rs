use std::{env, fs};
use std::path::PathBuf;
use poise::{ApplicationContext, BoxFuture, Command, SlashArgError};
use poise::serenity_prelude::{AutocompleteChoice, CommandOptionType, CreateAutocompleteResponse};
use reqwest::Client;
use songbird::input::Input;
use crate::types::{CommandInfo, Context, BotData, Error};
use crate::voice::play;

/// Plays stupid voice stuff
#[poise::command(slash_command, prefix_command)]
pub async fn sound(
    ctx: Context<'_>,
    #[description = "What to be played"]
    text: String,
    #[description = "In which language"]
    lang: Option<String>,
    #[description = "Gender"] gender: Option<String>,
) -> Result<(), Error> {
    let client = Client::new();
    let tts_key = env::var("TTS_KEY")?;
    let tts_url = env::var("TTS_URL")?;

    ctx.defer().await?;
    
    let response = client
        .get(tts_url)
        .query(&[
            ("text", text.clone()),
            ("lang", lang.unwrap_or_else(|| "cs".into())),
            ("gender", gender.unwrap_or_else(|| "female".into())),
            ("engine", "g1".into()),
            ("key", tts_key),
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
            Err("Text to speech request failed.".into())
        }
    }
}


async fn execute_voice_command(ctx: Context<'_>, text: String, input: Input) -> Result<(), Error> {
    play(ctx, input).await?;
    ctx.send(poise::CreateReply::default()
        .content(text)
        .ephemeral(true)
    ).await?;
    Ok(())
}

async fn slash_action_generic_command(ctx: Context<'_>, option: Option<String>) -> Result<(), Error> {
    ctx.defer().await?;
    let command_name = ctx.command().name.clone();
    let Some(command_options) = &ctx.data().audio_map.get(&command_name) else {
        return Err(format!("Command {} asset not found", command_name).into())
    };
    
    let path: Result<&PathBuf, Error> = match command_options { 
        CommandInfo::Path(path) => {
            Ok(path)
        },
        CommandInfo::Options(options) => {
            let Some(option_resolved) = option.clone() else {
                return Err("Missing option.".into())
            };
            let Some(path) = options.get(&option_resolved) else {
                return Err(format!("Option {} not found", option_resolved).into())
            };
            
            Ok(path)
            
        }
    };
    
    let data = fs::read(path?.clone())?;
    let input = songbird::input::Input::from(data);

    execute_voice_command(ctx, format!("{} {}", command_name, option.unwrap_or("".into())), input).await?;

    Ok(())
}

fn create_generic_asset_command_flat(command_name: String) -> Command<BotData, Error> {
    Command {
        identifying_name: command_name.clone(),
        name: command_name.clone(),
        qualified_name: command_name.clone(),
        source_code_name: command_name.clone(),
        description: Some(format!("Plays {}", command_name).into()),
        ephemeral: true,
        parameters: vec!(),
        slash_action: Some(|ctx| Box::pin(async move {
            slash_action_generic_command(ctx.into(), None)
                .await
                .map_err(|error| poise::FrameworkError::new_command(
                    ctx.into(),
                    error,
                ))
        })),
        ..Default::default()
    }
}

fn create_generic_asset_command_option(command_name: String) -> Command<BotData, Error> {
    Command {
        identifying_name: command_name.clone(),
        name: command_name.clone(),
        qualified_name: command_name.clone(),
        source_code_name: command_name.clone(),
        description: Some(format!("Plays {}", command_name).into()),
        ephemeral: true,
        parameters: vec![poise::CommandParameter {
            name: "option".into(),
            name_localizations: Default::default(),
            description: Some("Choose an option".into()),
            choices: vec!(),
            autocomplete_callback: Some(generic_asset_command_autocomplete),
            required: true,
            channel_types: None,
            type_setter: Some(|option| option.kind(CommandOptionType::String)),
            description_localizations: Default::default(),
            __non_exhaustive: (),
        }],
        slash_action: Some(|ctx| Box::pin(async move {
            let (option,) = poise::parse_slash_args!(ctx.serenity_context, ctx.interaction, ctx.args => ("option": Option<String>))
                .await
                .map_err(|error| error.to_framework_error(ctx))?;

            slash_action_generic_command(ctx.into(), option)
                .await
                .map_err(|error| poise::FrameworkError::new_command(
                    ctx.into(),
                    error,
                ))
        })),
        ..Default::default()
    }
}

/// Creates a generic asset command. The asset is read from the assets folder and has to follow a strict structure
pub fn create_generic_asset_command(
    command_name: String,
    command_info: &CommandInfo
    
) -> Command<BotData, Error> {
    match command_info {
        CommandInfo::Path(_) => {
            create_generic_asset_command_flat(command_name)
        },
        CommandInfo::Options(_) => {
            create_generic_asset_command_option(command_name)
        }
    }
}

fn generic_asset_command_autocomplete<'a>(
    ctx: ApplicationContext<'a, BotData, Error>,
    input: &'a str,
) -> BoxFuture<'a, Result<CreateAutocompleteResponse, SlashArgError>> {
    Box::pin(async move {
        let command_name = ctx.command().name.clone();
        let all_clips = &ctx.data().audio_map;
        let Some(command_def) = all_clips.get(&command_name) else {
            return Ok(CreateAutocompleteResponse::default())
        };
        match command_def {
            CommandInfo::Path(_) => {
                return Ok(CreateAutocompleteResponse::default())
            },
            CommandInfo::Options(options) => {
                let choices: Vec<_> = options
                    .keys()
                    .into_iter()
                    .filter_map(|value| value.contains(input).then(|| AutocompleteChoice::new(value.to_string(), value.to_string())))
                    .collect();

                Ok(CreateAutocompleteResponse::default().set_choices(choices))
            }
        }
        
    })
}