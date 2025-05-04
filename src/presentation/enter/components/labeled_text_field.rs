use dioxus::prelude::*;
use dioxus_material_icons::*;

#[derive(Clone, PartialEq, Props)]
pub struct LabeledTextFieldProperties {
    pub placeholder: String,
    pub label: String,
    pub text: String,
    pub on_input: EventHandler<FormEvent>,
    pub is_error: bool,
    pub icon: String,
}

#[allow(non_snake_case)]
pub fn LabeledTextField(properties: LabeledTextFieldProperties) -> Element {
    let mut is_focused = use_signal(|| false);

    rsx! {
        div {
            class: "text-field-container",
            width: "100%",
            margin: 0,
            padding: "10px 0",

            div { class: "input-wrapper",
                input {
                    class: if properties.is_error { "input-field error" } else { "input-field" },
                    r#type: "text",
                    value: "{properties.text}",
                    placeholder: if is_focused() { properties.placeholder } else { "" },
                    oninput: properties.on_input,
                    onfocusin: move |_| is_focused.set(true),
                    onfocusout: move |_| is_focused.set(false),
                    spellcheck: false,
                }

                label { class: "input-label", "{properties.label}" }

                div { class: "field-icon",
                    MaterialIcon {
                        name: properties.icon,
                        size: 28,
                        color: MaterialIconColor::Custom(String::from("rgb(64, 71, 81)")),
                    }
                }
            }

            style { r#"
                .text-field-container {{
                    position: relative;
                    margin: 24px 0;
                }}
                .input-wrapper {{
                    position: relative;
                }}
                .input-field {{
                    font-family: sans-serif;
                    width: 100%;
                    padding: 16px 44px 16px 12px;
                    border: 2px solid rgb(111, 119, 129);
                    border-radius: 10px;
                    font-size: 16px;
                    background: transparent;
                    outline: none;
                    color: rgb(69, 76, 86);
                    transition: all 0.3s;
                    box-sizing: border-box;
                }}
                .input-field:focus {{
                    border-color: #8fb2fd;
                }}
                .input-field.error {{
                    border-color: #ff3b30;
                }}
                .input-label {{
                    font-family: sans-serif;
                    position: absolute;
                    left: 12px;
                    color: rgb(69, 76, 86);
                    transform: translateY(100%);
                    pointer-events: none;
                    transition: all 0.3s cubic-bezier(0.2, 0, 0, 1);
                    max-width: calc(100% - 50px);
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }}
                .input-field:focus + .input-label,
                .input-field:not(:empty) + .input-label,
                .input-field:not(:placeholder-shown) + .input-label {{
                    top: -6px;
                    transform: translateY(0) scale(0.85);
                    font-size: 12px;
                    background: rgb(246, 247, 253);
                    padding: 0 4px;
                    left: 8px;
                    max-width: calc(100% - 56px);
                }}
                .input-field:focus + .input-label {{
                    color: #8fb2fd;
                }}
                .input-field.error + .input-label {{
                    color: #ff3b30;
                }}
                .field-icon {{
                    position: absolute;
                    right: 10px;
                    top: 50%;
                    transform: translateY(-50%);
                    pointer-events: none;
                }}
            "# }
        }
    }
}
