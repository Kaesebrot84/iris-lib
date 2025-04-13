use std::fmt;

use super::{Color, RGBA};

/// Represents a color, holding red, green, blue values as `u8` each.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn from_values(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}

impl From<RGBA> for RGB {
    fn from(rgba: RGBA) -> Self {
        RGB::from_values(rgba.r, rgba.g, rgba.b)
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ R: {}, G: {}, B: {} }}", self.r, self.g, self.b)
    }
}

/// Represents possible color channels in a RGB color.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RGBChannel {
    R,
    G,
    B,
}

impl Color for RGB {
    fn new() -> Self {
        Self::default()
    }

    fn complementary(&self) -> Self {
        Self::from_values(255 - self.r, 255 - self.g, 255 - self.b)
    }

    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    fn from_hex(_hex: &str) -> Self {
        todo!()
    }

    fn from_rgb(rgb: RGB) -> Self {
        rgb
    }

    fn from_rgba(rgba: RGBA) -> Self {
        RGB::from_values(rgba.r, rgba.g, rgba.b)
    }
}

impl Default for RGB {
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default() }
    }
}

/// Indexing implementation for the `RGB` struct using `RGBChannel` as index.
///
/// # Examples
///
/// ```
/// use iris_lib::color::{RGB, RGBChannel};
///
/// let color = RGB { r: 1, g: 2, b: 3 };
/// assert_eq!(1, color[RGBChannel::R]);
/// assert_eq!(2, color[RGBChannel::G]);
/// assert_eq!(3, color[RGBChannel::B]);
/// ```
///
impl ::std::ops::Index<RGBChannel> for RGB {
    type Output = u8;
    fn index(&self, index: RGBChannel) -> &Self::Output {
        match index {
            RGBChannel::R => &self.r,
            RGBChannel::G => &self.g,
            RGBChannel::B => &self.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_index_ut() {
        let rgb = RGB { r: 1, g: 2, b: 3 };
        assert_eq!(1, rgb[RGBChannel::R]);
        assert_eq!(2, rgb[RGBChannel::G]);
        assert_eq!(3, rgb[RGBChannel::B]);
    }

    #[test]
    fn rgb_new_ut() {
        let rgb = RGB::new();
        assert_eq!(0, rgb[RGBChannel::R]);
        assert_eq!(0, rgb[RGBChannel::G]);
        assert_eq!(0, rgb[RGBChannel::B]);
    }

    #[test]
    fn rgb_default_ut() {
        let rgb = RGB::default();
        assert_eq!(0, rgb[RGBChannel::R]);
        assert_eq!(0, rgb[RGBChannel::G]);
        assert_eq!(0, rgb[RGBChannel::B]);
    }

    #[test]
    fn rgb_to_hex_ut() {
        let mut rgb = RGB::new();

        assert_eq!("#000000", rgb.to_hex());

        rgb = RGB::from_values(255, 255, 255);
        assert_eq!("#FFFFFF", rgb.to_hex());

        rgb = RGB::from_values(89, 181, 138);
        assert_eq!("#59B58A", rgb.to_hex());

        rgb = RGB::from_values(86, 76, 92);
        assert_eq!("#564C5C", rgb.to_hex());
    }
}
