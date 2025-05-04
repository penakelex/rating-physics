use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone)]
pub struct PracticalLesson {
    pub not_attend: bool,
    pub tasks: Option<u8>,
}
