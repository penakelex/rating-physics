use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ListDataViewProperties {
    pub list_data: Vec<(String, String)>,
    pub is_name_and_value_same_size: bool,
    pub name: Option<String>,
}

#[allow(non_snake_case)]
pub fn ListDataView(properties: ListDataViewProperties) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            width: "100%",
            height: "calc(100vh - 180px)",

            if let Some(ref name) = properties.name {
                p {
                    margin: "0",
                    padding: "0",
                    padding_bottom: "8px",

                    font_family: "sans-serif",
                    font_size: "16px",
                    font_weight: "600",
                    color: "rgb(64, 71, 81)",

                    "{name}"
                }
            }

            div {
                width: "100%",
                overflow_x: "hidden",
                overflow_y: "auto",

                for (name, value) in properties.list_data.iter() {
                    p {
                        margin: "0",
                        padding: "4px 0",
                        font_family: "sans-serif",
                        color: "rgb(64, 71, 81)",

                        span {
                            font_weight: "600",
                            padding_right: "4px",
                            font_size: {
                                if properties.is_name_and_value_same_size { "15px" } else { "16px" }
                            },

                            "{name}"
                        }

                        span {
                            font_size: "15px",
                            "{value}"
                        }
                    }
                }
            }
        }
    }
}
