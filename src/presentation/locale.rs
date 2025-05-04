use fluent_templates::{static_loader, Loader};

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "ru-RU",
    };
}

pub fn get_language_string_resource(language: Language, identifier: &str) -> String {
    match language {
        Language::Russian => LOCALES.lookup(&"ru-RU".parse().unwrap(), identifier),
    }
}

pub fn get_string_resource(identifier: &str) -> String {
    match Language::default() {
        Language::Russian => LOCALES.lookup(&"ru-RU".parse().unwrap(), identifier),
    }
}

#[derive(Clone, Copy, Default)]
pub enum Language {
    #[default]
    Russian,
}