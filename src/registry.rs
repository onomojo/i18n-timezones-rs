use std::collections::HashMap;
use std::fmt;

use i18n_timezones_data::ALL_LOCALES;

#[derive(Debug)]
pub enum Error {
    LocaleNotFound(String),
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LocaleNotFound(l) => write!(f, "locale not found: {l}"),
            Error::ParseError(e) => write!(f, "parse error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

pub struct Registry {
    translations: HashMap<String, HashMap<String, String>>,
    default_locale: Option<String>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
            default_locale: None,
        }
    }

    pub fn register_locale(&mut self, locale: &str) -> Result<(), Error> {
        if self.translations.contains_key(locale) {
            return Ok(());
        }
        let json_str = ALL_LOCALES
            .iter()
            .find(|(l, _)| *l == locale)
            .map(|(_, data)| *data)
            .ok_or_else(|| Error::LocaleNotFound(locale.to_string()))?;
        let map: HashMap<String, String> = serde_json::from_str(json_str)
            .map_err(|e| Error::ParseError(e.to_string()))?;
        self.translations.insert(locale.to_string(), map);
        Ok(())
    }

    pub fn register_all_locales(&mut self) -> Result<(), Error> {
        for (locale, data) in ALL_LOCALES {
            if self.translations.contains_key(*locale) {
                continue;
            }
            let map: HashMap<String, String> = serde_json::from_str(data)
                .map_err(|e| Error::ParseError(e.to_string()))?;
            self.translations.insert(locale.to_string(), map);
        }
        Ok(())
    }

    pub fn set_default_locale(&mut self, locale: &str) -> Result<(), Error> {
        if !self.translations.contains_key(locale) {
            return Err(Error::LocaleNotFound(locale.to_string()));
        }
        self.default_locale = Some(locale.to_string());
        Ok(())
    }

    pub fn default_locale(&self) -> Option<&str> {
        self.default_locale.as_deref()
    }

    pub fn get_translation(&self, key: &str) -> Option<&str> {
        let locale = self.default_locale.as_deref()?;
        self.get_translation_for_locale(locale, key)
    }

    pub fn get_translation_for_locale(&self, locale: &str, key: &str) -> Option<&str> {
        self.translations
            .get(locale)?
            .get(key)
            .map(|s| s.as_str())
    }

    pub fn available_locales() -> &'static [&'static str] {
        static LOCALES: OnceLock<Vec<&'static str>> = OnceLock::new();
        let v = LOCALES.get_or_init(|| ALL_LOCALES.iter().map(|(l, _)| *l).collect());
        v.as_slice()
    }

    pub fn registered_locales(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.translations.keys().map(|s| s.as_str()).collect();
        v.sort();
        v
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

use std::sync::OnceLock;
