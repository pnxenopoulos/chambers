# chambers, a NYC subway crate
`chambers` is a Rust 🦀 library meant to monitor NYC subway station arrivals. It can be used to light up an LED board, where the LEDs light up/pulse when a train is arriving. We use the name "chambers", as the Chambers St. station is by far the most Rust-y station in the system!

### Development
```bash
cargo fmt --all --
cargo clippy -- -D warnings --allow dead_code
```

Here are some useful links:

GTFS Realtime specification: https://gtfs.org/documentation/realtime/proto/
MTA specification: https://api.mta.info/nyct-subway.proto.txt
Python version: https://github.com/Andrew-Dickinson/nyct-gtfs
List of stops: https://github.com/Andrew-Dickinson/nyct-gtfs/blob/master/nyct_gtfs/gtfs_static/stops.txt