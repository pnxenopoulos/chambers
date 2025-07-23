/// A single 24‑bit RGB color (8 bits per channel) for driving an LED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Create a new `Rgb` from individual channel values.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Return the color scaled by `brightness` (0.0 – 1.0).
    /// Useful when you have global‑brightness control on the strip.
    pub fn scale(self, brightness: f32) -> Self {
        debug_assert!(
            (0.0..=1.0).contains(&brightness),
            "brightness must be in [0, 1]"
        );
        // Use f32 math, then round to nearest integer.
        let scale = |c: u8| (c as f32 * brightness).round() as u8;
        Self {
            r: scale(self.r),
            g: scale(self.g),
            b: scale(self.b),
        }
    }

    /// Encode the color as a 24‑bit little‑endian integer (0x00RRGGBB).
    pub const fn to_be24(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }

    /// Return an 0xRRGGBB hex string (e.g. "#1A2B3C").
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Nice ergonomic conversions
impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

/// Physical LED position on the board.
#[derive(Clone, Copy, Debug)]
pub struct LedAddr {
    /// Which LED strip (one per subway line).
    pub strip_index: usize,
    /// The pixel’s offset *within* that strip.
    pub pixel_index: usize,
}

/// Colors
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Blue,
    Orange,
    LightGreen,
    Brown,
    Grey,
    Yellow,
    Red,
    DarkGreen,
    Purple,
    Teal,
}

impl Color {
    /// Get the `Rgb` for this variant.
    pub const fn rgb(self) -> Rgb {
        match self {
            Color::Blue => Rgb::new(0, 98, 207),
            Color::Orange => Rgb::new(238, 104, 0),
            Color::LightGreen => Rgb::new(121, 149, 52),
            Color::Brown => Rgb::new(142, 92, 51),
            Color::Grey => Rgb::new(124, 133, 140),
            Color::Yellow => Rgb::new(246, 188, 38),
            Color::Red => Rgb::new(216, 34, 51),
            Color::DarkGreen => Rgb::new(0, 153, 82),
            Color::Purple => Rgb::new(154, 56, 161),
            Color::Teal => Rgb::new(0, 142, 183),
        }
    }

    /// Convenience: apply brightness directly on the enum.
    pub fn with_brightness(self, b: f32) -> Rgb {
        self.rgb().scale(b)
    }

    /// Emoji
    pub const fn emoji(self) -> &'static str {
        match self {
            Color::Blue => "🔵",
            Color::Orange => "🟠",
            Color::LightGreen => "🟢",
            Color::Brown => "🟤",
            Color::Grey => "⚪️",
            Color::Yellow => "🟡",
            Color::Red => "🔴",
            Color::DarkGreen => "🟢",
            Color::Purple => "🟣",
            Color::Teal => "🩵",
        }
    }
}

/// Let you do `Rgb::from(Color::Red)` or `Color::Red.into()`.
impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        c.rgb()
    }
}
