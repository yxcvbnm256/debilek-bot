use std::collections::HashMap;
use poise::Command;
use poise::serenity_prelude::{GuildId, VoiceState};
use crate::types::{Error, Context};

pub trait ContextExt {
    fn get_voice_channel(&self) -> Result<(VoiceState, GuildId), Error>;
}

impl ContextExt for Context<'_> {
    /// Gets the data about a voice channel from a command context.
    /// TODO handle the guild id
    fn get_voice_channel(&self) -> Result<(VoiceState, GuildId), Error> {
        let user_id = self.author().id;
        let guild = if let Some(guild_ref) = self.guild() {
            guild_ref.clone()
        } else {
            return Err("Piča však to ani není v guildě.".into());
        };
        let Some((_, voice_channel)) = guild
            .voice_states
            .iter()
            .filter(|(_key, channel)| channel.user_id == user_id)
            .last() else { return Err("Hele debílku jeden, jednou sem ti to toleroval, ale teď už to vážně není vtipný. Okamžitě se přidej do voice channelu, nebo ti nechám zrušit celej kanál.".into()); };
        Ok((voice_channel.clone(), guild.id))
    }
}

pub trait HashSetExt {
    fn get_fitting_keys(&self, partial: &str) -> Vec<String>; }

impl HashSetExt for HashMap<&str, &str> {
    /// Gets the keys of the hashset that contain the partial string.
    /// Used for command autocomplete
    fn get_fitting_keys(&self, partial: &str) -> Vec<String> {
        self.iter()
            .filter(|(_, value)| value.contains(partial))
            .map(|(key, _)| key.to_string())
            .collect()
    }
}