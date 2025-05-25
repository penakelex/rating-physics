#![windows_subsystem = "windows"]

use std::rc::Rc;

use crate::data::repository::error::GetRatingDataError;
use data::repository::RatingRepository;
use dioxus::desktop::Config;
use dioxus::desktop::LogicalSize;
use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;
use dioxus_i18n::prelude::use_init_i18n;
use dioxus_i18n::prelude::I18nConfig;
use dioxus_i18n::unic_langid::langid;
use dioxus_material_icons::*;

use presentation::*;

pub mod data;
pub mod domain;
pub mod presentation;

#[cfg(windows)]
fn app_config() -> Config {
    use dioxus::desktop::tao::{platform::windows::IconExtWindows, window::Icon};

    Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("RatingPhysics")
                .with_min_inner_size(LogicalSize::new(700, 500)),
        )
        .with_icon(Icon::from_path("assets/icon.ico", None).unwrap())
        .with_menu(None)
}

#[cfg(not(windows))]
fn app_config() -> Config {
    Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("RatingPhysics")
                .with_min_inner_size(LogicalSize::new(700, 500)),
        )
        .with_menu(None)
}

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(app_config())
        .launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    let mut data = use_signal(|| DataState::NotYetSearched);

    let repository = RatingRepository::new();

    use_init_i18n(|| {
        I18nConfig::new(langid!("ru-RU")).with_locale((
            langid!("ru-RU"),
            include_str!("data/locales/ru-RU/main.ftl"),
        ))
    });

    let latest_version_future = {
        let repository = repository.clone();
        use_resource(move || {
            let repository = repository.clone();
            async move { repository.get_latest_version().await }
        })
    };

    let callback = use_callback({
        let repository = repository.clone();
        move |get_data: GetRatingData| {
            data.set(DataState::Loading);

            let repository = repository.clone();

            spawn(async move {
                let file_bytes = std::fs::read(get_data.file_path.as_str()).unwrap();

                data.set(
                    match repository
                        .get_rating_data(get_data.password, file_bytes, get_data.file_type)
                        .await
                    {
                        Ok(rating_data) => DataState::LoadedData(Rc::new(rating_data)),

                        Err(error) => match error {
                            GetRatingDataError::CantAccesServer => DataState::CantAccessServer,
                            GetRatingDataError::InvalidPassword => DataState::NoLoadedData,
                            GetRatingDataError::InvalidRatingDataFormat => {
                                eprintln!("Invalid rating data format");
                                DataState::NoLoadedData
                            }
                        },
                    },
                );
            });
        }
    });

    let version = latest_version_future.read_unchecked();

    rsx! {
        MaterialIconStylesheet {
            variant: MaterialIconVariant::SelfHosted(
                asset!("/assets/MaterialIcons-Regular.woff2").to_string(),
            ),
        }

        div {
            position: "fixed",
            top: 0,
            left: 0,
            width: "100%",
            height: "100%",
            padding: 0,
            background: "rgb(246, 247, 253)",
            display: "flex",
            flex_direction: "column",

            font_family: "sans-serif",

            EnterView {
                on_search: callback,
            }

            RatingView {
                data: data(),
            }

            InformationView {
                version: version.as_ref().unwrap_or(&None).as_ref().map(|version| version.clone()),
            }
        }
    }
}
