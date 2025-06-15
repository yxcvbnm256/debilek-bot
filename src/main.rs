mod types;
mod enums;
mod extensions;
mod asset_processing;
mod commands;
mod voice;

use std::env;
use dotenv::dotenv;
use poise::{serenity_prelude as serenity};
use songbird::SerenityInit;

use crate::asset_processing::{choose_greetings};
use crate::enums::VoiceChannelAction;
use crate::types::Error;
use crate::types::Data;
use crate::voice::{get_voice_channel_action, play_serenity};


#[tokio::main]
async fn main() {
    let songbird = songbird::Songbird::serenity();
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .expect("Invalid discord token.");
    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::sound(),
                commands::franta(),
                commands::zesrane(),
                commands::dufka(),
                commands::cojetypico(),
                commands::misc(),
                commands::dota(),
            ],
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
                Ok(Data {})
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
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::VoiceStateUpdate {old, new} => {

            match get_voice_channel_action(old, new, ctx) {
                VoiceChannelAction::None => {
                    return Ok(())
                },
                // greet the user
                VoiceChannelAction::UserJoined => {
                    println!("User {:?} joined voice channel {:?}.", new.user_id, new.channel_id);
                    let src = choose_greetings(&new.user_id)?;
                    let res = play_serenity(ctx, new, src).await;
                    return res
                },
                // fuck off if nobody in the channel
                VoiceChannelAction::UserLeftEmptyChannel => {
                    println!("User {:?} left voice channel {:?}, which is now empty.", new.user_id, new.channel_id);
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