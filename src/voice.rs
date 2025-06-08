use crate::types::{Context, Error};
use crate::extensions::{ContextExt};
use poise::{serenity_prelude as serenity};
use poise::serenity_prelude::VoiceState;
use songbird::input::Input;
use crate::enums::VoiceChannelAction;

/// Plays a songbird input to a voice channel.
pub async fn play_serenity(
    ctx: &serenity::Context,
    voice_channel: &VoiceState,
    input: Input,
) -> Result<(), Error> {
    let Some(guild_id) = voice_channel.guild_id else { return Err("No guild ID.".into())};
    let Some(channel_id) = voice_channel.channel_id else { return Err("No channel ID.".into())};
    let manager = songbird::get(&ctx)
        .await
        .ok_or("Failed to get Songbird voice client")?;
    match manager.join(guild_id, channel_id).await {
        Ok(handler_lock) => {
            let mut handler = handler_lock.lock().await;
            let _ = handler.play_input(input);
            println!("Successfully joined voice channel {:?}", channel_id);
            Ok(())
        }
        Err(e) => {
            println!("Failed to join voice channel {:?}", channel_id);
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Plays a songbird input to a voice channel.
pub async fn play(
    ctx: Context<'_>,
    input: Input,
) -> Result<(), Error> {
    let voice_channel = ctx.get_voice_channel().or_else(|e| Err(e))?;
    play_serenity(ctx.serenity_context(), &voice_channel, input).await
}

/// Determines the action that should be taken when a VoiceStateUpdate occurs.
pub fn get_voice_channel_action(
    old: &Option<VoiceState>,
    new: &VoiceState,
    ctx: &serenity::Context) -> VoiceChannelAction {

    let self_user_id = ctx.cache.current_user().id;

    // do not react to yourself
    if ctx.cache.current_user().id == new.user_id {
        return VoiceChannelAction::None;
    }

    // an old state exists -> user did not join from fuck-know-where but from a different channel
    if let Some(old_channel) = old {
        // no real change happens, for example, if a user starts streaming
        if old_channel.channel_id == new.channel_id || new.channel_id.is_some() {            
            VoiceChannelAction::None
        } else {
            // user might have left
            let Some(old_guild_id) = old_channel.guild_id else { return VoiceChannelAction::None; };
            let Some(old_voice_channel_id) = old_channel.channel_id else { return VoiceChannelAction::None; };
            let Some(guild) = old_guild_id.to_guild_cached(ctx) else { return VoiceChannelAction::None; };

            let members_in_channel_count = guild
                .voice_states
                .iter()
                .filter(|(key, channel)| channel.channel_id == Some(old_voice_channel_id) && **key != self_user_id)
                .count();

            // user left, and nobody else is in the channel -> fuck off
            if members_in_channel_count == 0 {
                VoiceChannelAction::UserLeftEmptyChannel
            } else {
                VoiceChannelAction::None
            }
        }
    } else if new.channel_id.is_some() {
        return VoiceChannelAction::UserJoined;
    } else {
        return VoiceChannelAction::None;
    }
}