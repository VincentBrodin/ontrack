use serde::de::DeserializeOwned;
use std::{
    fs,
    io::{self, Read},
    path::Path,
};
use thiserror::Error;

mod config;
pub mod models;
pub use config::*;
use models::{GtfsAgency, GtfsArea, GtfsRoute, GtfsStop, GtfsStopArea, GtfsTransfer};

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("Csv error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Csv file {0} is missing header")]
    MissingHeader(String),
}

/// Used to load GTFS3 data into memory
#[derive(Default)]
pub struct Gtfs {
    pub(crate) stops: Vec<GtfsStop>,
    pub(crate) areas: Vec<GtfsArea>,
    pub(crate) routes: Vec<GtfsRoute>,
    pub(crate) agency: Vec<GtfsAgency>,
    pub(crate) stop_areas: Vec<GtfsStopArea>,
    pub(crate) transfers: Vec<GtfsTransfer>,
    config: Config,
}

impl Gtfs {
    pub fn new(config: self::Config) -> Self {
        Self {
            stops: Default::default(),
            areas: Default::default(),
            routes: Default::default(),
            agency: Default::default(),
            stop_areas: Default::default(),
            transfers: Default::default(),
            config,
        }
    }

    pub fn load_from_zip<P: AsRef<Path>>(mut self, path: P) -> Result<Self, self::Error> {
        let file = fs::File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name();
            match name {
                val if val == self.config.stops_file_name => parse_csv(&mut self.stops, &mut file)?,
                val if val == self.config.areas_file_name => parse_csv(&mut self.areas, &mut file)?,
                val if val == self.config.routes_file_name => {
                    parse_csv(&mut self.routes, &mut file)?
                }
                val if val == self.config.agency_file_name => {
                    parse_csv(&mut self.agency, &mut file)?
                }
                val if val == self.config.stop_areas_file_name => {
                    parse_csv(&mut self.stop_areas, &mut file)?
                }
                val if val == self.config.transfers_file_name => {
                    parse_csv(&mut self.transfers, &mut file)?
                }
                _ => {
                    #[cfg(debug_assertions)]
                    println!("Missed {name}")
                }
            };
        }
        Ok(self)
    }

    pub fn stops(&self) -> &Vec<GtfsStop> {
        &self.stops
    }

    pub fn areas(&self) -> &Vec<GtfsArea> {
        &self.areas
    }
    pub fn routes(&self) -> &Vec<GtfsRoute> {
        &self.routes
    }
    pub fn agency(&self) -> &Vec<GtfsAgency> {
        &self.agency
    }
    pub fn stop_areas(&self) -> &Vec<GtfsStopArea> {
        &self.stop_areas
    }
    pub fn transfers(&self) -> &Vec<GtfsTransfer> {
        &self.transfers
    }
}

fn parse_csv<R, T>(buf: &mut Vec<T>, reader: &mut R) -> Result<(), self::Error>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let record: T = result?;
        buf.push(record);
    }
    Ok(())
}
