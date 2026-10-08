# Interface translations

`en.json` is the English catalog; `zh-CN.json` contains Simplified Chinese translations.
Use stable semantic keys when adding text. `translate(locale, key)` falls back to English
when a Chinese translation is missing, then to the key when neither catalog has it.
`translate_label` bridges existing English settings labels during incremental migration.
Do not translate storage keys, settings section slugs, action identifiers, or shell commands.

GUI Settings uses the local `general.interface_language` preference (`system`, `en`, or
`zh-CN`). Change it under Settings → Account → Interface language, or through the command
palette. Settings navigation, page titles, and category headers are cataloged centrally, so
they translate as soon as a catalog entry exists. The Account, Appearance, Privacy, Scripting,
Knowledge, Warp Drive, Referrals, Warpify, Keyboard shortcuts, Shared blocks, Third party CLI
agents, Codebase Indexing, Editor and Code Review, Profiles, Cloud Environments, and API keys
pages translate their labels, descriptions, enumerated controls, buttons, empty states, and
toasts; migrated widgets also answer Chinese search terms. Unmigrated pages and widgets
continue to display their existing English text. Catalog values may embed `{placeholder}`
tokens that call sites substitute for dynamic values. `settings_text` falls back to English
when the locale settings model is not registered, so rendering stays safe in test harnesses.
The shared catalog API is available to both front-ends; TUI text has not yet been migrated.

The same preference can be set in `settings.toml`:

```toml
[general]
interface_language = "zh-CN"
```

Add matching keys to both catalogs and extend the catalog tests when introducing behavior.
System locale is detected once per process; restart after changing the OS language.
