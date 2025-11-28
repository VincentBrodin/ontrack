pub struct Config {
    pub stops_file_name: String,
    pub areas_file_name: String,
    pub routes_file_name: String,
    pub agency_file_name: String,
    pub stop_areas_file_name: String,
    pub transfers_file_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stops_file_name: "stops.txt".into(),
            areas_file_name: "areas.txt".into(),
            routes_file_name: "routes.txt".into(),
            agency_file_name: "agency.txt".into(),
            stop_areas_file_name: "stop_areas.txt".into(),
            transfers_file_name: "transfers.txt".into(),
        }
    }
}
