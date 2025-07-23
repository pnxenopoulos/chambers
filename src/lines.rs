use crate::led::{Color, LedAddr, Rgb};

#[derive(Debug)]
pub struct Station {
    pub stop_id: &'static str, // e.g. "A24"
    pub name: &'static str,    // "34 St ‑ Penn Station"
    pub gps: (f64, f64),       // optional but handy for hacks
}

#[derive(Debug)]
pub struct StationInLine {
    pub station: &'static Station,
    pub led: LedAddr,
}

#[derive(Debug)]
pub struct Line {
    pub route_id: &'static str, // "A", "C", "E", "N", …
    pub color: Rgb,
    pub stations: &'static [StationInLine],
}

use phf::phf_map;

pub static STOPS: phf::Map<&'static str, Color> = phf_map! {
    "1" => Color::Red,
    "2" => Color::Red,
    "3" => Color::Red,
    "4" => Color::DarkGreen,
    "5" => Color::DarkGreen,
    "6" => Color::DarkGreen,
    "7" => Color::Purple,
    "A" => Color::Blue,
    "B" => Color::Orange,
    "C" => Color::Blue,
    "D" => Color::Orange,
    "E" => Color::Blue,
    "F" => Color::Orange,
    "G" => Color::LightGreen,
    "J" => Color::Brown,
    "L" => Color::Grey,
    "M" => Color::Orange,
    "N" => Color::Yellow,
    "Q" => Color::Yellow,
    "R" => Color::Yellow,
    "H" => Color::Grey,
    "FS" => Color::Grey,
    "GS" => Color::Grey,
    "W" => Color::Yellow,
    "Z" => Color::Brown,
};

pub fn route_emoji(route_id: &str) -> &'static str {
    STOPS.get(route_id).map(|c| c.emoji()).unwrap_or("❓")
}
