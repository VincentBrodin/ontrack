use std::sync::Arc;

use crate::gtfs::models::GtfsArea;

pub struct Area {
    pub id: Arc<str>,
    pub name: Arc<str>,
}

impl From<GtfsArea> for Area {
    fn from(value: GtfsArea) -> Self {
        Self {
            id: value.area_id.into(),
            name: value.area_name.into(),
        }
    }
}
