use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locale {
    English,
    SimplifiedChinese,
}

impl Locale {
    pub fn resolve(preference: &str, system_locale: Option<&str>) -> Self {
        let code = if preference == "system" {
            system_locale.unwrap_or("en")
        } else {
            preference
        };
        if code.eq_ignore_ascii_case("zh")
            || code.to_ascii_lowercase().starts_with("zh-")
            || code.to_ascii_lowercase().starts_with("zh_")
        {
            Self::SimplifiedChinese
        } else {
            Self::English
        }
    }
}

static ENGLISH: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("en.json")).expect("invalid English translation catalog")
});
static CHINESE: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("zh-CN.json")).expect("invalid Chinese translation catalog")
});

pub fn translate(locale: Locale, key: &str) -> &str {
    let translated = match locale {
        Locale::English => ENGLISH.get(key),
        Locale::SimplifiedChinese => CHINESE.get(key).or_else(|| ENGLISH.get(key)),
    };
    translated.map(String::as_str).unwrap_or(key)
}

/// Translates legacy settings labels while they are migrated to catalog keys.
pub fn translate_label(locale: Locale, label: &str) -> &str {
    let key = ENGLISH
        .iter()
        .find_map(|(key, value)| (value == label).then_some(key));
    key.map(|key| translate(locale, key)).unwrap_or(label)
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
