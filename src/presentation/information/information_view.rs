use dioxus::prelude::*;
use dioxus_i18n::t;

const APP_VERSION: &str = "1.1.0";

const UPDATE_URL: &str = "http://45.90.46.187:8000/rating_physics/get_apps";
const TELEGRAM_URL: &str = "https://t.me/penakelex";

static TELEGRAM_ICON: Asset = asset!("/assets/telegram.png");

#[derive(Clone, PartialEq, Props)]
pub struct InformationViewProperties {
    pub version: Option<String>,
}

#[allow(non_snake_case)]
pub fn InformationView(properties: InformationViewProperties) -> Element {
    rsx! {
        div {
            display: "flex",
            width: "100%",
            padding: "6px",
            flex_direction: "row",
            justify_content: "space-between",
            align_items: "center",
            box_sizing: "border-box",

            {match properties.version {
                Some(ref version) if version != APP_VERSION => rsx! {
                    a {
                        href: UPDATE_URL,

                        text_decoration: "none",
                        color: "rgb(64, 71, 81)",
                        margin: 0,

                        span {
                            font_weight: "600",
                            font_size: "12px",
                            padding_right: "4px",
                            {t!("update_app_label")}
                        }

                        span {
                            font_size: "10px",
                            font_style: "italic",
                            "{APP_VERSION} -> {version}"
                        }
                    }
                },

                _ => rsx! {
                    div {
                        flex_grow: 1,
                    }
                }
            }}

            p {
                color: "rgb(64, 71, 81)",
                margin: 0,

                span {
                    font_weight: "600",
                    font_size: "12px",
                    padding_right: "8px",
                    {t!("table_developer_label")}
                }

                span {
                    font_size: "10px",
                    padding_right: "12px",
                    {t!("table_developer_name")}
                }
            }

            a {
                href: TELEGRAM_URL,

                text_decoration: "none",
                color: "rgb(64, 71, 81)",
                margin: 0,

                span {
                    font_weight: "600",
                    font_size: "12px",
                    padding_right: "8px",
                    {t!("contact_app_developer_label")}
                }

                span {
                    font_size: "10px",
                    padding_right: "12px",
                    {t!("app_developer_name")}
                }

                img {
                    src: TELEGRAM_ICON,
                    vertical_align: "middle",
                    alt: "Telegram icon",
                    width: "18px",
                    height: "18px",
                }
            }
        }
    }
}
