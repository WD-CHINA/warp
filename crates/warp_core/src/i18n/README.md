# Interface translations

`en.json` is the English catalog; `zh-CN.json` contains Simplified Chinese translations.
Use stable semantic keys when adding text. `translate(locale, key)` falls back to English
when a Chinese translation is missing, then to the key when neither catalog has it.
`translate_label` bridges existing English settings labels during incremental migration.
Do not translate storage keys, settings section slugs, action identifiers, or shell commands.

GUI Settings uses the local `general.interface_language` preference (`system`, `en`, or
`zh-CN`). Change it under Settings → Account → Interface language, or through the command
palette. Settings navigation, cataloged page titles, account actions, version update messages,
scripting installation controls, and the language row update immediately. Account and scripting
widgets also include Chinese search terms. Privacy and Appearance now include translated
labels, descriptions, category headings, and enumerated controls; these update when the
language changes, including the custom redaction modal.
Unmigrated widgets continue to display their existing English text. The shared catalog API
is available to both front-ends; TUI text has not yet been migrated.

The same preference can be set in `settings.toml`:

```toml
[general]
interface_language = "zh-CN"
```

Add matching keys to both catalogs and extend the catalog tests when introducing behavior.
System locale is detected once per process; restart after changing the OS language.
