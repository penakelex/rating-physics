use std::rc::Rc;

use crate::domain::model::rating_data::RatingData;

#[derive(Clone, PartialEq)]
pub enum DataState {
    CantAccessServer,
    Loading,
    LoadedData(Rc<RatingData>),
    NoLoadedData,
    NotYetSearched,
}
