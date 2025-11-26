use csv::Reader;
use serde::de::DeserializeOwned;
use std::{
    fs::{self},
    io::Read,
    path::Path,
};

use crate::gtfs::{self, GtfsAgency, GtfsArea, GtfsRoute, GtfsStop, GtfsStopArea, GtfsTransfer};

#[derive(Default)]
pub struct GtfsLoader {
    pub stops: Vec<GtfsStop>,
    pub areas: Vec<GtfsArea>,
    pub routes: Vec<GtfsRoute>,
    pub agency: Vec<GtfsAgency>,
    pub stop_areas: Vec<GtfsStopArea>,
    pub transfers: Vec<GtfsTransfer>,
}

impl GtfsLoader {
    pub fn from_zip<P: AsRef<Path>>(path: P) -> Result<Self, gtfs::Error> {
        let file = fs::File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        let mut out = Self::default();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name();
            match name {
                "stops.txt" => parse_csv(&mut out.stops, &mut file)?,
                "areas.txt" => parse_csv(&mut out.areas, &mut file)?,
                "routes.txt" => parse_csv(&mut out.routes, &mut file)?,
                "agency.txt" => parse_csv(&mut out.agency, &mut file)?,
                "stop_areas.txt" => parse_csv(&mut out.stop_areas, &mut file)?,
                "transfers.txt" => parse_csv(&mut out.transfers, &mut file)?,
                _ => println!("Missed {name}"),
            };
        }
        Ok(out)
    }
}

fn parse_csv<R, T>(buf: &mut Vec<T>, reader: &mut R) -> Result<(), gtfs::Error>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut rdr = Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let record: T = result?;
        buf.push(record);
    }
    Ok(())
}
