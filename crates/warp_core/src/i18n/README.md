# Interface translations

`en.json` is the English catalog; `zh-CN.json` contains Simplified Chinese translations.
Use stable semantic keys when adding text. `translate(locale, key)` falls back to English
when a Chinese translation is missing, then to the key when neither catalog has it.
`translate_label` bridges existing English settings labels during incremental migration.
Do not translate storage keys, settings section slugs, action identifiers, or shell commands.

GUI Settings uses the local `general.interface_language` preference (`system`, `en`, or
`zh-CN`). Change it under Settings → Account → Interface language, or through the command
palette. Every Settings page is migrated: labels, descriptions, enumerated controls, buttons,
placeholders, empty and error states, toasts, and tooltips render through `settings_text`, and
migrated widgets answer Chinese search terms. Navigation items, page titles, and category
headers are cataloged centrally, so they translate as soon as an entry exists. Catalog values
may embed `{placeholder}` tokens that call sites substitute for dynamic values.
`settings_text` falls back to English when the locale settings model is not registered, so
rendering stays safe in test harnesses.

Text that intentionally stays English: brand and product names, plan and model identifiers,
technical values (shells, GPU backends, key names, API-key examples), command-palette toggle
labels (the palette keeps English labels, and account settings append Chinese aliases), the
`SettingsFileError` banner copy shared with the workspace banner, and the About-page copyright
line. The shared catalog API is available to both front-ends; TUI text has not yet been
migrated.

The same preference can be set in `settings.toml`:

```toml
[general]
interface_language = "zh-CN"
```

Add matching keys to both catalogs and extend the catalog tests when introducing behavior.
System locale is detected once per process; restart after changing the OS language.
