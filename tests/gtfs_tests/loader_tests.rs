use ontrack::gtfs;

#[test]
fn load_from_zip_test() {
    let zip_path = format!("{}/tests/gtfs3.zip", env!("CARGO_MANIFEST_DIR"));
    let loader = gtfs::GtfsLoader::from_zip(zip_path).unwrap();

    for stop in loader.stops.iter() {
        if stop.stop_id.is_empty() {
            panic!("stop_id should never be null");
        }
        if stop.stop_name.is_empty() {
            panic!("stop_name should never be null");
        }
    }

    for area in loader.areas.iter() {
        if area.area_id.is_empty() {
            panic!("area_id should never be null");
        }
        if area.area_name.is_empty() {
            panic!("area_name should never be null");
        }

        if area.samtrafiken_area_type.is_empty() {
            panic!("samtrafiken_area_type should never be null");
        }
    }

    for agency in loader.agency.iter() {
        if agency.agency_id.is_empty() {
            panic!("agency_id should never be null");
        }
        if agency.agency_name.is_empty() {
            panic!("agency_name should never be null");
        }
        if agency.agency_url.is_empty() {
            panic!("agency_url should never be null");
        }
        if agency.agency_timezone.is_empty() {
            panic!("agency_timezone should never be null");
        }
        if agency.agency_lang.is_empty() {
            panic!("agency_lang should never be null");
        }
    }

    for route in loader.routes.iter() {
        if route.route_id.is_empty() {
            panic!("route_id should never be null");
        }
        if route.agency_id.is_empty() {
            panic!("agency_id should never be null");
        }
    }

    for stop_area in loader.stop_areas.iter() {
        if stop_area.stop_id.is_empty() {
            panic!("stop_id should never be null");
        }
        if stop_area.area_id.is_empty() {
            panic!("area_id should never be null");
        }
    }

    for transfer in loader.transfers.iter() {
        if transfer.from_stop_id.is_empty() {
            panic!("from_stop_id should never be null");
        }
        if transfer.to_stop_id.is_empty() {
            panic!("to_stop_id should never be null");
        }
        if transfer.transfer_type.is_empty() {
            panic!("transfer_type should never be null");
        }
    }
}
