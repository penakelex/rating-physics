use std::path::PathBuf;

#[allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_material_icons::*;
use rfd::AsyncFileDialog;

use crate::{data::repository::ciphered_file_type::CipheredFileType, enter::components::*};

#[derive(Clone)]
pub struct GetRatingData {
    pub password: u32,
    pub file_path: String,
    pub file_type: CipheredFileType,
}

#[derive(Clone, PartialEq, Props)]
pub struct EnterViewProperties {
    pub on_search: Callback<GetRatingData>,
}

#[allow(non_snake_case)]
pub fn EnterView(properties: EnterViewProperties) -> Element {
    let mut password = use_signal(String::new);
    let mut is_password_correct = use_signal(|| true);

    let mut file_path = use_signal(String::new);
    let mut is_file_valid = use_signal(|| true);
    let mut is_picking_file = use_signal(|| false);

    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            gap: "12px",
            padding_left: "12px",
            padding_right: "12px",
            box_sizing: "border-box",
            width: "100%",

            div {
                flex: "1 1 0",
                min_width: 0,

                LabeledTextField {
                    label: t!("password_label"),
                    placeholder: t!("password_hint"),
                    text: password,
                    on_input: move |event: Event<FormData>| {
                        let new_password = event.value();

                        is_password_correct.set(
                            new_password.is_ascii()
                            && new_password.len() <= 5
                            && new_password.bytes().all(|symbol| symbol.is_ascii_digit())
                        );

                        password.set(new_password);
                    },
                    is_error: !is_password_correct(),
                    icon: "password",
                }
            }

            div {
                flex: "1 1 0",
                min_width: 0,

                div {
                    width: "100%",
                    flex: "column",

                    style {
                        r#"
                            .button {{
                                width: 100%;
                                background: rgb(0, 95, 156);
                                border-radius: 8px;
                                border: none;
                                color: #f8f9ffff;
                                padding: 10px;
                                font-size: 16px;
                                font-family: sans-serif;
                                animation: ripple 0.6s linear;
                            }}
                            .button:hover {{
                                background: rgb(20, 108, 164);
                            }}
                            .button:active {{
                                transform: translateY(1px) scale(0.99);
                                box-shadow: 0 1px 2px rgba(0,0,0,0.2);
                            }}
                        "#
                    }

                    LabeledTextField {
                        label: t!("file_label"),
                        placeholder: t!("file_hint"),
                        text: file_path,
                        on_input: move |event: Event<FormData>| {
                            file_path.set(event.value());
                            is_file_valid.set(true);
                        },
                        is_error: !is_file_valid(),
                        icon: "insert_drive_file",
                    }

                    button { class: "button",
                        onclick: move |_| {
                            if is_picking_file() {
                                return;
                            }

                            is_picking_file.set(true);

                            spawn(async move {
                                if let Some(file) = AsyncFileDialog::new()
                                    .add_filter(t!("file_label"), &["rpf", "zip"])
                                    .pick_file()
                                    .await
                                {
                                    if let Some(file_path_from_dialog) = file.path().to_str() {
                                        file_path.set(file_path_from_dialog.to_string());
                                        is_file_valid.set(true);
                                    }
                                }

                                is_picking_file.set(false);
                            });
                        },

                        {t!("file_selection_button_label")}
                    }
                }
            }

            div {
                flex: "1 1 0",
                min_width: 0,
                margin_top: "auto",

                button { class: "button",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    gap: "4px",

                    onclick: move |_| {
                        let password = password();
                        let file_path = file_path();

                        if !password.is_ascii() || !(4..=5).contains(&password.len())
                            || password.bytes().any(|symbol| !symbol.is_ascii_digit())
                        {
                            is_password_correct.set(false);
                        }

                        let path = PathBuf::from(file_path.as_str());
                        let Some(extension) = path.extension() else {
                            is_file_valid.set(false);
                            return;
                        };

                        if !path.exists() || extension != "rpf" && extension != "zip" {
                            is_file_valid.set(false);
                        }

                        if !is_password_correct() || !is_file_valid() {
                            return;
                        }

                        let Some(file_type) = CipheredFileType::try_from_str(extension.to_str().unwrap_or("")) else {
                            return;
                        };

                        properties.on_search.call(GetRatingData {
                            password: password.parse::<u32>().unwrap(),
                            file_path,
                            file_type,
                        });
                    },

                    MaterialIcon {
                        name: "search",
                        size: 16,
                    }

                    {t!("search_button_label")}
                }
            }
        }
    }
}
