use i18n_timezones::Registry;

#[test]
fn test_register_and_translate() {
    let mut reg = Registry::new();
    reg.register_locale("en").unwrap();
    reg.register_locale("ja").unwrap();
    reg.set_default_locale("en").unwrap();

    assert_eq!(reg.get_translation("Tokyo"), Some("Tokyo"));
    assert!(reg.get_translation_for_locale("ja", "Tokyo").is_some());
}

#[test]
fn test_register_all() {
    let mut reg = Registry::new();
    reg.register_all_locales().unwrap();
    assert!(reg.registered_locales().len() >= 36);
}

#[test]
fn test_available_locales() {
    let locales = Registry::available_locales();
    assert!(locales.len() >= 36);
    assert!(locales.contains(&"en"));
    assert!(locales.contains(&"ja"));
}

#[test]
fn test_global_api() {
    i18n_timezones::register_locale("en").unwrap();
    i18n_timezones::set_default_locale("en").unwrap();
    assert_eq!(i18n_timezones::get_translation("Tokyo"), Some("Tokyo".to_string()));
}

#[test]
fn test_unregistered_locale() {
    let mut reg = Registry::new();
    assert!(reg.set_default_locale("zz").is_err());
    assert!(reg.get_translation_for_locale("zz", "Tokyo").is_none());
}
