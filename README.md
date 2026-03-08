# i18n-timezones-rs

Rust crate for localized timezone name translations. Covers 152 ActiveSupport timezones across 36 locales, sourced from Unicode CLDR.

All translation data is embedded at compile time — zero runtime I/O, no external files needed.

## Install

```toml
[dependencies]
i18n-timezones = "0.1"
```

## Usage

```rust
use i18n_timezones::Registry;

let mut reg = Registry::new();
reg.register_locale("ja").unwrap();
reg.set_default_locale("ja").unwrap();

assert_eq!(reg.get_translation("Tokyo"), Some("東京"));
```

### Global API

```rust
i18n_timezones::register_locale("en").unwrap();
i18n_timezones::set_default_locale("en").unwrap();
let name = i18n_timezones::get_translation("Tokyo"); // Some("Tokyo")
```

## License

MIT
