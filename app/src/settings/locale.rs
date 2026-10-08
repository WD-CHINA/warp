use std::sync::LazyLock;

use settings::macros::define_settings_group;
use settings::{Setting, SupportedPlatforms, SyncToCloud};
use warp_core::i18n::{Locale, translate, translate_label};
use warpui::{AppContext, SingletonEntity};

define_settings_group!(LocaleSettings, settings: [
    interface_language: InterfaceLanguage {
        type: String,
        default: "system".to_string(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        surface: settings::SettingSurfaces::ALL,
        private: false,
        storage_key: "InterfaceLanguage",
        toml_path: "general.interface_language",
        description: "Interface language: system, en, or zh-CN. Untranslated text falls back to English.",
    }
]);

static SYSTEM_LOCALE: LazyLock<Option<String>> = LazyLock::new(sys_locale::get_locale);

fn interface_locale(ctx: &AppContext) -> Locale {
    if !ctx.has_singleton_model::<LocaleSettings>() {
        return Locale::English;
    }
    let preference = LocaleSettings::as_ref(ctx).interface_language.value();
    Locale::resolve(preference, SYSTEM_LOCALE.as_deref())
}

pub fn interface_text<'a>(key: &'a str, ctx: &AppContext) -> &'a str {
    translate(interface_locale(ctx), key)
}

pub fn settings_text<'a>(label: &'a str, ctx: &AppContext) -> &'a str {
    translate_label(interface_locale(ctx), label)
}
