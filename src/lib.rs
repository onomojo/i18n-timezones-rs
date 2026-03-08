mod generated;
mod registry;

pub use registry::{Error, Registry};

use std::sync::{Mutex, OnceLock};

static GLOBAL: OnceLock<Mutex<Registry>> = OnceLock::new();

fn global() -> &'static Mutex<Registry> {
    GLOBAL.get_or_init(|| Mutex::new(Registry::new()))
}

/// Register a locale in the global registry.
pub fn register_locale(locale: &str) -> Result<(), Error> {
    global().lock().unwrap().register_locale(locale)
}

/// Register all available locales in the global registry.
pub fn register_all_locales() -> Result<(), Error> {
    global().lock().unwrap().register_all_locales()
}

/// Set the default locale in the global registry.
pub fn set_default_locale(locale: &str) -> Result<(), Error> {
    global().lock().unwrap().set_default_locale(locale)
}

/// Get a translation using the global registry's default locale.
pub fn get_translation(key: &str) -> Option<String> {
    let reg = global().lock().unwrap();
    reg.get_translation(key).map(|s: &str| s.to_string())
}

/// Get a translation for a specific locale using the global registry.
pub fn get_translation_for_locale(locale: &str, key: &str) -> Option<String> {
    let reg = global().lock().unwrap();
    reg.get_translation_for_locale(locale, key).map(|s: &str| s.to_string())
}

/// List all available locales in the embedded data.
pub fn available_locales() -> &'static [&'static str] {
    Registry::available_locales()
}
