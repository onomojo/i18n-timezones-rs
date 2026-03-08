# i18n-timezones

[![Crates.io](https://img.shields.io/crates/v/i18n-timezones.svg)](https://crates.io/crates/i18n-timezones)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Localized timezone names for Rust -- 36 locales, 152 timezones, zero runtime I/O.

Building a timezone picker? Displaying meeting times across regions? Your users expect to see **"Eastern Time (US & Canada)"** -- in their own language, not as a raw IANA identifier.

**i18n-timezones** provides human-friendly, localized timezone display names sourced from [CLDR](https://cldr.unicode.org/), the same data that powers ICU, Chrome, and Android. All translation data is embedded at compile time via `include_str!` -- no external files, no filesystem access, no network calls. Your binary is fully self-contained.

## Why i18n-timezones?

- **36 locales** covering 4+ billion speakers -- from Arabic to Vietnamese
- **Compile-time embedded** -- all data baked into the binary, zero runtime I/O
- **Two APIs** -- owned `Registry` struct for testability, or global convenience functions for simplicity
- **Thread-safe globals** -- global API is protected by `Mutex`, safe for concurrent use
- **No panic** -- all lookups return `Option`, never panics on missing data
- **Minimal dependencies** -- only `serde_json` and `once_cell`

## Install

```toml
[dependencies]
i18n-timezones = "0.1"
```

## Quick Start

```rust
use i18n_timezones::Registry;

let mut reg = Registry::new();
reg.register_locale("ja").unwrap();
reg.set_default_locale("ja").unwrap();

assert_eq!(reg.get_translation("Tokyo"), Some("東京"));
```

## Usage

### Struct-based API (recommended)

The `Registry` struct gives you an owned, testable translation store:

```rust
use i18n_timezones::Registry;

let mut reg = Registry::new();
reg.register_locale("de").unwrap();
reg.register_locale("fr").unwrap();
reg.set_default_locale("de").unwrap();

// Lookup with default locale
let name = reg.get_translation("Tokyo");
assert_eq!(name, Some("Tokio"));

// Lookup with explicit locale
let name = reg.get_translation_for_locale("fr", "Tokyo");
assert_eq!(name, Some("Tokyo"));

// List what's loaded
let locales = reg.registered_locales(); // ["de", "fr"]
```

### Register all locales at once

```rust
let mut reg = Registry::new();
reg.register_all_locales().unwrap();
// All 36 locales now available
```

### Global convenience API

For simpler use cases where you don't want to pass a `Registry` around:

```rust
i18n_timezones::register_locale("de").unwrap();
i18n_timezones::set_default_locale("de").unwrap();

let name = i18n_timezones::get_translation("Tokyo");
assert_eq!(name, Some("Tokio".to_string()));

// Explicit locale
let name = i18n_timezones::get_translation_for_locale("de", "Berlin");
assert_eq!(name, Some("Berlin".to_string()));
```

### List available locales

```rust
// All locales in the embedded data (no registration needed)
let available = i18n_timezones::available_locales();
assert!(available.len() >= 36);
assert!(available.contains(&"ja"));
```

## API Reference

### `Registry` (struct-based)

| Method | Description |
|--------|-------------|
| `Registry::new()` | Create a new empty registry. |
| `register_locale(&mut self, locale)` | Load translations for a single locale. No-op if already registered. |
| `register_all_locales(&mut self)` | Load all 36 available locales. |
| `set_default_locale(&mut self, locale)` | Set the default locale. Returns `Err` if not registered. |
| `default_locale(&self)` | Get the current default locale. |
| `get_translation(&self, key)` | Get the localized name using the default locale. |
| `get_translation_for_locale(&self, locale, key)` | Get the localized name for a specific locale. |
| `available_locales()` | List all locales in the embedded data (static). |
| `registered_locales(&self)` | List all currently loaded locales. |

### Global functions

| Function | Description |
|----------|-------------|
| `register_locale(locale)` | Register a locale in the global registry. |
| `register_all_locales()` | Register all locales in the global registry. |
| `set_default_locale(locale)` | Set the global default locale. |
| `get_translation(key)` | Translate using the global default locale. Returns `Option<String>`. |
| `get_translation_for_locale(locale, key)` | Translate for a specific locale. Returns `Option<String>`. |
| `available_locales()` | List all available locales. |

All lookups return `None` when a timezone or locale is not found -- no panics.

## Supported Locales

36 locales covering major world languages:

| | | | | | | |
|---|---|---|---|---|---|---|
| ar | bn | ca | cs | da | de | el |
| en | es | eu | fi | fr | he | hi |
| hr | hu | id | it | ja | ko | ms |
| nl | no | pl | pt | pt-BR | ro | ru |
| sq | sv | th | tr | uk | vi | zh-CN |
| zh-TW | | | | | | |

## Data Source

All translations come from the [Unicode CLDR](https://cldr.unicode.org/) (Common Locale Data Repository) -- the industry-standard source used by every major platform including iOS, Android, Chrome, and Java. This ensures translations are accurate, consistent, and maintained by native speakers through Unicode's established review process.

## Also Available For

- **[Ruby](https://github.com/onomojo/i18n-timezones)** -- Rails gem with automatic `time_zone_select` integration
- **[JavaScript/TypeScript](https://github.com/onomojo/i18n-timezones-js)** -- NPM package with tree-shaking and dropdown helpers
- **[Go](https://github.com/onomojo/i18n-timezones-go)** -- Go module with embedded data via `go:embed`

## License

MIT
