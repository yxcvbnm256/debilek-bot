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

/// Přehraje pičovinu
#[poise::command(slash_command, prefix_command)]
pub async fn sound(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"] text: String,
    #[description = "v jakém jazyku"] lang: Option<String>,
) -> Result<(), Error> {
    println!("přehrávám {}", text);

    let req = reqwest::get(format!("https://translate.google.com.vn/translate_tts?ie=UTF-8&q={}&tl={}&client=tw-ob'", text, lang.unwrap_or_else(|| "cs".to_string())))
        .await?
        .error_for_status();
    match req {
        Ok(r) => {
            println!("Status code is {:?}", r.status().as_str());
            let src = songbird::input::Input::from(r.bytes().await?);
            play(ctx, src).await.or_else(|e| Err(e))?;
            ctx.reply(text).await?;
            Ok(())
        },
        Err(e) => {
            println!("{:?}", e.status().unwrap().as_str());
            Err("Google translate mě poslal do kokotu, sorry.".into())
        }
    }
}

/// Přehraje cojetypíčho
#[poise::command(slash_command, prefix_command)]
pub async fn cojetypico(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"]
    #[autocomplete = "cojetypico_autocomplete"]
    option: String,
) -> Result<(), Error> {
    let asset = get_asset_file(AssetClass::Cojetypico, option.as_str())?;
    play(ctx, asset).await.or_else(|e| Err(e))?;
    ctx.reply(option).await?;
    Ok(())
}

/// Přehraje frantu
#[poise::command(slash_command, prefix_command)]
pub async fn franta(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"]
    #[autocomplete = "franta_autocomplete"]
    option: String,
) -> Result<(), Error> {
    let asset = get_asset_file(AssetClass::Franta, option.as_str())?;
    play(ctx, asset).await.or_else(|e| Err(e))?;
    ctx.reply(option).await?;
    Ok(())
}

/// Přehraje zesrané hajzle
#[poise::command(slash_command, prefix_command)]
pub async fn zesrane(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"]
    #[autocomplete = "zesrane_autocomplete"]
    option: String,
) -> Result<(), Error> {
    let asset = get_asset_file(AssetClass::ZesraneHajzle, option.as_str())?;
    play(ctx, asset).await.or_else(|e| Err(e))?;
    ctx.reply(option).await?;
    Ok(())
}

/// Přehraje viktora
#[poise::command(slash_command, prefix_command)]
pub async fn dufka(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"]
    #[autocomplete = "dufka_autocomplete"]
    option: String,
) -> Result<(), Error> {
    let asset = get_asset_file(AssetClass::Dufka, option.as_str())?;
    play(ctx, asset).await.or_else(|e| Err(e))?;
    ctx.reply(option).await?;
    Ok(())
}

/// Přehraje whatever
#[poise::command(slash_command, prefix_command)]
pub async fn misc(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"]
    #[autocomplete = "misc_autocomplete"]
    option: String,
) -> Result<(), Error> {
    let asset = get_asset_file(AssetClass::Misc, option.as_str())?;
    play(ctx, asset).await.or_else(|e| Err(e))?;
    ctx.reply(option).await?;
    Ok(())
}

/// Přehraje dota mrtku
#[poise::command(slash_command, prefix_command)]
pub async fn dota(
    ctx: Context<'_>,
    #[description = "co chceš přehrát"]
    #[autocomplete = "dota_autocomplete"]
    option: String,
) -> Result<(), Error> {
    let asset = get_asset_file(AssetClass::Dota, option.as_str())?;
    play(ctx, asset).await.or_else(|e| Err(e))?;
    ctx.reply(option).await?;
    Ok(())
}