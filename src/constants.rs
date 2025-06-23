use crate::types::Context;

const LANG_CODES: &[&'static str] = &[
    "af-ZA", "ar-SA", "bn-BD", "bn-IN", "ca-ES", "cs-CZ", "da-DK", "de-DE", "el-GR", "en-AU", 
    "en-GB", "en-IE", "en-In", "en-IN", "en-NG", "en-US", "en-ZA", "es-ES", "es-MX", "es-US",
    "et-EE", "fi-FI", "fr-CA", "fr-FR", "gu-IN", "he-IL", "hi-IN", "hr-HR", "hu-HU", "hy-AM", 
    "id-ID", "in-ID", "is-IS", "it-IT", "ja-JP", "jv-ID", "km-KH", "kn-IN", "ko-KR", "Li-mu", 
    "lv-LV", "mk-MK", "ml-IN", "mr-IN", "ms-MY", "nb-NO", "ne-NP", "nl-BE", "nl-NL", "no-NO",
    "pl-PL", "pt-BR", "pt-PT", "ro-RO", "ru-RU", "si-LK", "sk-SK", "sq-AL", "sr-ME", "sr-RS", 
    "su-ID", "sv-SE", "sw-KE", "ta-IN", "te-IN", "th-TH", "tr-TR", "uk-UA", "ur-PK", "vi-VN",
    "zh-CN", "zh-HK", "zh-TW" ];

pub async fn languages_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<String> {
    LANG_CODES.iter().filter(|&s| s.starts_with(partial)).map(|s| s.to_string()).collect()
}

const TTL_VOICES: &[&'static str] = &[ "male", "female" ];

pub async fn voices_autocomplete(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<String> {
    TTL_VOICES.iter().filter(|&s| s.starts_with(partial)).map(|s| s.to_string()).collect()
}