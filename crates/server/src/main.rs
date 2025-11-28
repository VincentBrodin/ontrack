use std::{env, path::Path, process::exit, time::Instant};

use ontrack::{engine, gtfs};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Missing gtfs zip");
        exit(1);
    }

    let path = Path::new(&args[1]).canonicalize().unwrap();

    let data = gtfs::Gtfs::new(gtfs::Config::default())
        .load_from_zip(path)
        .unwrap();
    let engine = engine::Engine::new().with_gtfs(data);

    let start = Instant::now();
    let area = engine.get_area(&args[2]).unwrap();
    let stops = engine.get_stops_in_area(&args[2]).unwrap();
    let duration = start.elapsed();
    println!("{} links to {}", area.id, area.name);
    for stop in stops.iter() {
        println!("  {} type: {:?}", stop.name, stop.location_type);
    }
    println!("Operation took: {:?}", duration);
}
