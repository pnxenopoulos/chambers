/// (human‑readable label, full GTFS‑realtime URL)
pub const GTFS_RT_FEEDS: [(&str, &str); 7] = [
    // Includes Rockaway shuttle (H)
    (
        "ace",
        "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-ace",
    ),
    // Includes Franklin shuttle (FS)
    (
        "bdfm",
        "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-bdfm",
    ),
    (
        "g",
        "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-g",
    ),
    (
        "jz",
        "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-jz",
    ),
    (
        "l",
        "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-l",
    ),
    (
        "nqrw",
        "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-nqrw",
    ),
    // Includes Grand Central Shuttle (GS)
    (
        "1234567S",
        "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs",
    ),
];
