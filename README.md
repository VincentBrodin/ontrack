# On Track

On Track is a high-performance Rust library for loading, routing, searching, and querying GTFS transit data,
designed for minimal runtime allocations and fast lookups.

> [!INFO]
> This project is early in development, if you like the idea and want to help improve it, please open an issue.

## Instalation
```bash
cargo add ontrack
```

## Design
The `ontrack` `engine` is designed to be **immutable** and **thread-safe**. All data is stored only once on the heap, and the engine works by passing around references rather than cloning or reallocating existing structures.


Entities such as `stops`, `areas`, and others are stored as `Arc<[T]>` slices, and all strings are held as `Arc<str>`. This ensures thread safety and keeps the memory footprint low, since no entity is ever allocated more than once.


The only time the `engine` allocates new memory is when performing request-driven operations such as **search** operations. In those cases, the newly allocated memory is owned entirely by the consumer (i.e., you). In a scenario like a web server, this means the allocated data exists only for the duration of the request and is freed immediately afterward.

## Implemented
- Load GTFS data directly from `.zip` archives.
- In-memory GTFS engine for fast read/query operations.
- Direct querying of entities by ID. *500ns to 500µs*
- Fuzzy search for stops and geographic areas.

## Roadmap
- Distance-based search for stops and areas.
- Simple distance-based routing.
- Time-based routing and schedule-aware journey planning.

## Usage
Simple program for finding areas (An area is a geografic area/collection of stops).
```rust
use std::{env, path::Path, process::exit, time::Instant};

use ontrack::{
    engine::{self, Identifiable},
    gtfs,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Missing gtfs zip and/or search string");
        exit(1);
    }

    let path = Path::new(&args[1]).canonicalize().unwrap();

    let data = gtfs::Gtfs::new(gtfs::Config::default())
        .load_from_zip(path)
        .unwrap();
    let engine = engine::Engine::new().with_gtfs(data);

    let start = Instant::now();
    let results = engine.search_areas_by_name(&args[2]);
    for value in results.iter().take(5) {
        println!("{}", value.name());
    }
    let duration = start.elapsed();
    println!("Operation took: {:?}", duration);
}
```
## Refrences
- [GTFS Specification](https://gtfs.org/documentation/schedule/reference/)
- [Development Data (Sweden):](https://www.trafiklab.se/api/gtfs-datasets/gtfs-sweden/static-specification/)
