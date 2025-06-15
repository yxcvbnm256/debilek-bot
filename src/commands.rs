use std::env;
use reqwest::Client;
use crate::asset_processing::get_asset_file;
use crate::asset_processing::franta_autocomplete;
use crate::asset_processing::dufka_autocomplete;
use crate::asset_processing::zesrane_autocomplete;
use crate::asset_processing::misc_autocomplete;
use crate::asset_processing::cojetypico_autocomplete;
use crate::asset_processing::dota_autocomplete;
use crate::enums::AssetClass;
use crate::types::{Context, Error};
use crate::voice::play;

/// Plays stupid voice stuff
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

/// Plays dota bullshit
#[poise::command(slash_command, prefix_command)]
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