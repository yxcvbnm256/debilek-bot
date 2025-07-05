use std::collections::HashMap;
use poise::serenity_prelude::{GuildId, VoiceState};
use crate::types::{Error, Context, CommandInfo};

pub trait ContextExt {
    fn get_voice_channel_and_guild(&self) -> Result<(VoiceState, GuildId), Error>;
}

impl ContextExt for Context<'_> {
    /// Gets the data about a voice channel from a command context.
    fn get_voice_channel_and_guild(&self) -> Result<(VoiceState, GuildId), Error> {
        let user_id = self.author().id;
        let guild = if let Some(guild_ref) = self.guild() {
            guild_ref.clone()
        } else {
            return Err("You cannot use me outside of a guild.".into());
        };
        let Some((_, voice_channel)) = guild
            .voice_states
            .iter()
            .filter(|(_key, channel)| channel.user_id == user_id)
            .last() 
        else {
            return Err("Hele debílku jeden, jednou sem ti to toleroval, ale teď už to vážně není vtipný. Okamžitě se přidej do voice channelu, nebo ti nechám zrušit celej kanál.".into());
        };
        Ok((voice_channel.clone(), guild.id))
    }
}

impl Default for CommandInfo {
    fn default() -> Self {
        CommandInfo::Options(HashMap::new())
    }
}
