mod types;
mod enums;
mod traits;
mod asset_processing;
mod commands;
mod voice;
mod constants;

use std::env;
use std::path::Path;
use dotenv::dotenv;
use poise::{serenity_prelude as serenity};
use songbird::SerenityInit;

use crate::asset_processing::{choose_greetings, discover_audio_structure};
use crate::commands::{create_generic_asset_command};
use crate::enums::VoiceChannelAction;
use crate::types::{Config, Error};
use crate::types::BotData;
use crate::voice::{get_voice_channel_action, play_serenity};



#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .expect("Invalid discord token.");
    
    let config_raw = env::var("CONFIG")
        .expect("Config not provided.");
    let config: Config = serde_json::from_str(&config_raw).unwrap();
    
    let songbird = songbird::Songbird::serenity();
    
    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let assets_path = Path::new("assets");
    let audio_map = discover_audio_structure(assets_path);

    // Create dynamic commands from audio asset map
    let mut commands: Vec<_> = audio_map
        .iter()
        .map(|(command_name, command_info)| create_generic_asset_command(
            command_name.clone(),
            command_info,
        ))
        .collect();
    
    // Add static commands
    commands.push(commands::sound());
    
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default() 
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                // test for test guild
                //poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId::new(769146546905284608)).await?; 
                Ok(BotData { audio_map, config })
            })
            
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird_with(songbird)
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, BotData, Error>,
    data: &BotData,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        // Handles greetings - if a user joins a voice channel, play greeting
        serenity::FullEvent::VoiceStateUpdate {old, new} => {

            match get_voice_channel_action(old, new, ctx) {
                VoiceChannelAction::None => {
                    return Ok(())
                },
                // greet the user
                VoiceChannelAction::UserJoined => {
                    let src = choose_greetings(&new.user_id, data)?;
                    let res = play_serenity(ctx, new, None, src).await;
                    return res
                },
                // fuck off if nobody in the channel
                VoiceChannelAction::UserLeftEmptyChannel => {
                    let manager = songbird::get(ctx)
                        .await
                        .ok_or_else(|| "Failed to get Songbird voice client.")?;

                    // unwrap is safe because get_voice_channel_action checks for this
                    // anyway TODO think about better solution
                    let channel_id = old.as_ref().unwrap().channel_id.unwrap();
                    let guild_id = new.guild_id.unwrap();
                    match manager.leave(guild_id).await {
                        Ok(_) => println!("Leaving voice channel {:?}.", channel_id),
                        Err(err) => println!("Failed to leave voice channel {:?}: {:?}", channel_id, err),
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}