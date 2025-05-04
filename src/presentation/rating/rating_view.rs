use dioxus::prelude::*;

use crate::presentation::{locale::get_string_resource, rating::components::RatingDataView};

use super::DataState;

#[derive(Clone, Props, PartialEq)]
pub struct RatingViewProperties {
    pub data: DataState,
}

#[allow(non_snake_case)]
pub fn RatingView(properites: RatingViewProperties) -> Element {
    match properites.data {
        DataState::Loading => {
            rsx! {
                style {
                    r#"
                    .spinner-container {{
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                    }}

                    .spinner {{
                        width: 40px;
                        height: 40px;
                        border: 4px solid #f3f3f3;
                        border-top: 4px solid #3498db;
                        border-radius: 50%;
                        animation: spin 1s linear infinite;
                    }}

                    @keyframes spin {{
                        0% {{ transform: rotate(0deg); }}
                        100% {{ transform: rotate(360deg); }}
                    }}
                    "#
                }

                div {
                    flex_grow: 1,

                    class: "spinner-container",
                    div { class: "spinner" }
                }
            }
        }

        DataState::LoadedData(rating_data) => {
            rsx! {
                div {
                    width: "100%",
                    padding_top: "12px",
                    flex_grow: 1,

                    RatingDataView {
                        rating_data: rating_data,
                    }
                }
            }
        }

        DataState::NotYetSearched => {
            rsx! {
                div {
                    flex_grow: 1,
                }
            }
        }

        data => {
            rsx! {
                div {
                    display: "flex",
                    flex_grow: 1,
                    justify_content: "center",
                    align_items: "center",

                    p {
                        font_size: "18px",
                        overflow: "hidden",
                        font_family: "sans-serif",

                        {if matches!(data, DataState::CantAccessServer) {
                            get_string_resource("cant_access_server_label")
                        } else {
                            get_string_resource("student_with_such_password_not_found_label")
                        }}
                    }
                }
            }
        }
    }
}
