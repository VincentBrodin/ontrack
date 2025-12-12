use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct Transfer {
    pub from_stop_id: Arc<str>,
    pub from_stop_idx: usize,
    pub to_stop_id: Arc<str>,
    pub to_stop_idx: usize,

    pub from_trip_id: Option<Arc<str>>,
    pub from_trip_idx: Option<usize>,
    pub to_trip_id: Option<Arc<str>>,
    pub to_trip_idx: Option<usize>,

    pub min_transfer_time: Option<usize>,
}
