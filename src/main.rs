mod feeds;
mod led;
mod lines;
mod stations;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use gtfs_realtime::FeedMessage;
use prost::Message;
use reqwest::Client;
use tokio::time::{self, Duration as TokioDuration};

use crate::feeds::GTFS_RT_FEEDS;
use crate::lines::route_emoji;
use crate::stations::STOPS;

fn is_in_next_2_min(dt: DateTime<Utc>) -> bool {
    let now = Utc::now();
    dt > now && dt <= now + Duration::minutes(5)
}

fn list_arriving_train(msg: FeedMessage) {
    for entity in msg.entity {
        let Some(trip_update) = entity.trip_update else {
            continue;
        };
        if let Some(first_stop) = trip_update.stop_time_update.first() {
            if let (Some(arrival), Some(stop_id)) = (first_stop.arrival, &first_stop.stop_id) {
                if let Some(ts) = arrival.time {
                    if let Some(est_arrival_time) = DateTime::<Utc>::from_timestamp(ts, 0) {
                        if is_in_next_2_min(est_arrival_time) {
                            let route = trip_update.trip.route_id.as_deref().unwrap_or("?");
                            let emoji = route_emoji(route);
                            let stop_name = STOPS.get(stop_id).unwrap(); // whatever map you use for names
                            println!(
                                "{emoji} {route} train arriving at {stop_name} at {est_arrival_time}"
                            );
                        }
                    }
                }
            }
        }
    }
}

async fn fetch_and_print(url: &str, client: &Client) -> Result<()> {
    let bytes = client.get(url).send().await?.bytes().await?;
    let feed = FeedMessage::decode(bytes.as_ref())?;
    list_arriving_train(feed);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder().user_agent("Chambers Rust Bot").build()?;

    let mut interval = time::interval(TokioDuration::from_secs(30));
    // Consume the immediate first tick so the body runs right away.
    interval.tick().await;

    loop {
        // Optionally listen for Ctrl+C to exit cleanly
        tokio::select! {
            _ = interval.tick() => {
                for (_label, url) in &GTFS_RT_FEEDS {
                    if let Err(e) = fetch_and_print(url, &client).await {
                        eprintln!("Error fetching {url}: {e}");
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                eprintln!("Shutting down...");
                break;
            }
        }
    }

    Ok(())
}
