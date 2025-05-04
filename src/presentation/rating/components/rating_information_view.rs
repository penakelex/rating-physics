use std::{borrow::Cow, rc::Rc};

use dioxus::prelude::*;

use crate::{
    domain::model::{practical_lesson::PracticalLesson, rating_data::RatingData},
    presentation::{locale::get_string_resource, rating::components::list_data_view::ListDataView},
};

#[derive(Clone, PartialEq, Props)]
pub struct RatingDataViewProperties {
    pub rating_data: Rc<RatingData>,
}

#[allow(non_snake_case)]
pub fn RatingDataView(properties: RatingDataViewProperties) -> Element {
    let general_rating_data = list_rating_data(&properties.rating_data);

    rsx! {
        div {
            width: "100%",
            display: "flex",
            flex_direction: "row",
            gap: "8px",
            padding: "4px",

            div {
                flex: "1 1 0",

                ListDataView {
                    list_data: general_rating_data,
                    is_name_and_value_same_size: false,
                }
            }

            div {
                flex: "1 1 0",

                ListDataView {
                    list_data: {
                        properties.rating_data.practical_lessons.iter().enumerate()
                            .map(|(number, lesson)| (format!("{}.", number + 1), practical_lesson_to_string(&lesson)))
                            .collect()
                    },
                    is_name_and_value_same_size: true,
                    name: get_string_resource("practical_lessons_label"),
                }
            }

            div {
                flex: "1 1 0",

                ListDataView {
                    list_data: {
                        properties.rating_data.cgts.iter().enumerate()
                            .map(|(number, task)| (format!("{}.", number + 1), cgt_task_to_string(&task)))
                            .collect()
                    },
                    is_name_and_value_same_size: true,
                    name: get_string_resource("cgts_label"),
                }
            }
        }
    }
}

fn list_rating_data(data: &RatingData) -> Vec<(String, String)> {
    let colloquium = match data.colloquium {
        Some(mark) => Cow::Owned(mark.to_string()),
        None => Cow::Owned(get_string_resource("no_data_label")),
    };

    let lw = match data.lw {
        Some(score) => Cow::Owned(score.to_string()),
        None => Cow::Owned(get_string_resource("no_data_label")),
    };

    let it = match data.it {
        Some(mark) => Cow::Owned(mark.to_string()),
        None => Cow::Owned(get_string_resource("no_data_label")),
    };

    let essay = match data.essay {
        Some(mark) => Cow::Owned(mark.to_string()),
        None => Cow::Owned(get_string_resource("no_data_label")),
    };

    let nirs = match data.nirs {
        Some(mark) => Cow::Owned(mark.to_string()),
        None => Cow::Owned(get_string_resource("no_data_label")),
    };

    let list: Vec<(String, Cow<str>)> = vec![
        (get_string_resource("full_name_label"), Cow::Borrowed(&data.full_name)),
        (get_string_resource("group_label"), Cow::Borrowed(&data.group)),
        (get_string_resource("summary_label"), Cow::Owned(format!("{:.2}", data.summary))),
        (
            get_string_resource("rating_group_label"),
            Cow::Owned(data.rating_group.to_string()),
        ),
        (get_string_resource("rating_flow_label"), Cow::Owned(data.rating_flow.to_string())),
        (get_string_resource("colloquium_label"), colloquium),
        (
            get_string_resource("cgt_cw_label"),
            Cow::Owned(format!("{:.2}", data.cgt_cw)),
        ),
        (get_string_resource("lw_label"), lw),
        (get_string_resource("it_label"), it),
        (get_string_resource("essay_label"), essay),
        (get_string_resource("nirs_label"), nirs),
        (
            get_string_resource("sum_practice_label"),
            Cow::Owned(data.sum_practice.to_string()),
        ),
    ];

    list.into_iter()
        .map(|(name, value)| (format!("{name}:"), value.to_string()))
        .collect()
}

fn practical_lesson_to_string(lesson: &PracticalLesson) -> String {
    let mut builder = String::new();

    match lesson.tasks {
        Some(tasks) => {
            let tasks_string = tasks.to_string();
            builder.push_str(tasks_string.as_str());
        }
        None => builder.push_str(get_string_resource("no_data_label").as_str()),
    }

    if lesson.not_attend {
        builder.push(' ');
        builder.push_str(get_string_resource("was_not_on_lesson_label").as_str());
    }

    builder
}

fn cgt_task_to_string(tasks: &Option<u8>) -> String {
    match tasks {
        Some(tasks) => tasks.to_string(),
        None => get_string_resource("no_data_label"),
    }
}
