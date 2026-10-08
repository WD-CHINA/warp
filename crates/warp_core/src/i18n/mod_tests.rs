use super::*;

#[test]
fn resolves_system_and_explicit_preferences() {
    assert_eq!(
        Locale::resolve("system", Some("zh_CN.UTF-8")),
        Locale::SimplifiedChinese
    );
    assert_eq!(Locale::resolve("en", Some("zh-CN")), Locale::English);
    assert_eq!(
        Locale::resolve("zh-CN", Some("en-US")),
        Locale::SimplifiedChinese
    );
    assert_eq!(Locale::resolve("system", None), Locale::English);
    assert_eq!(Locale::resolve("unknown", Some("zh-CN")), Locale::English);
}

#[test]
fn translates_labels_and_preserves_unknown_text() {
    assert_eq!(
        translate_label(Locale::SimplifiedChinese, "Settings"),
        "设置"
    );
    assert_eq!(translate_label(Locale::English, "Settings"), "Settings");
    assert_eq!(
        translate_label(Locale::SimplifiedChinese, "Warp Drive"),
        "Warp Drive"
    );
    assert_eq!(
        translate(Locale::SimplifiedChinese, "missing.key"),
        "missing.key"
    );
}

#[test]
fn catalogs_have_matching_keys_and_unique_english_labels() {
    assert_eq!(ENGLISH.len(), CHINESE.len());
    let mut labels = std::collections::HashSet::new();
    for (key, label) in ENGLISH.iter() {
        assert!(CHINESE.get(key).is_some_and(|value| !value.is_empty()));
        assert!(labels.insert(label));
    }
}
