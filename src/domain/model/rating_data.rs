use serde::Deserialize;

use super::practical_lesson::PracticalLesson;

#[derive(Deserialize, PartialEq, Clone)]
pub struct RatingData {
    pub full_name: String,
    pub group: String,
    pub summary: f32,
    pub rating_group: u8,
    pub rating_flow: u16,
    pub colloquium: Option<u8>,
    pub cgt_cw: f32,
    pub lw: Option<u8>,
    pub it: Option<u8>,
    pub essay: Option<u8>,
    pub nirs: Option<u8>,
    pub sum_practice: u8,
    pub omissions: u8,
    pub practical_lessons: Vec<PracticalLesson>,
    pub cgts: Vec<Option<u8>>,
}