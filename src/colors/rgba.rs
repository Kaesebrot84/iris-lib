use std::fmt;

use super::{Color, RGB};

/// Represents a color, holding red, green, blue and alpha values as `u8` each.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn from_values(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA { r, g, b, a }
    }
}

impl From<RGB> for RGBA {
    // TODO: Tests
    fn from(rgba: RGB) -> Self {
        Self::from_values(rgba.r, rgba.g, rgba.b, 255)
    }
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ R: {}, G: {}, B: {}, A: {} }}", self.r, self.g, self.b, self.a)
    }
}

impl Default for RGBA {
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default(), a: 255 }
    }
}

impl Color for RGBA {
    fn new() -> Self {
        Self::default()
    }

    fn to_hex(&self) -> String {
        // TODO: Tests
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }

    fn from_hex(_hex: &str) -> Self {
        todo!()
    }

    // TODO: Tests
    // Does not affect alpha channel
    fn complementary(&self) -> Self {
        Self::from_values(255 - self.r, 255 - self.g, 255 - self.b, 255)
    }

    fn random() -> Self {
        // TODO: Test
        Self::from_values(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>())
    }
}

/// Indexing implementation for the `RGBA` struct using `RGBAChannel` as index.
/// TODO: Same for RGB
///
/// # Examples
///
/// ```
/// use iris_lib::colors::{RGBA, RGBAChannel};
///
/// let color = RGBA { r: 1, g: 2, b: 3, a: 4 };
/// assert_eq!(1, color[RGBAChannel::R]);
/// assert_eq!(2, color[RGBAChannel::G]);
/// assert_eq!(3, color[RGBAChannel::B]);
/// assert_eq!(4, color[RGBAChannel::A]);
/// ```
///
impl ::std::ops::Index<RGBAChannel> for RGBA {
    type Output = u8;
    fn index(&self, index: RGBAChannel) -> &Self::Output {
        match index {
            RGBAChannel::R => &self.r,
            RGBAChannel::G => &self.g,
            RGBAChannel::B => &self.b,
            RGBAChannel::A => &self.a,
        }
    }
}

/// Represents possible color channels in a RGBA color.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RGBAChannel {
    R,
    G,
    B,
    A,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgba_index_ut() {
        let color = RGBA::random();
        assert_eq!(color.r, color[RGBAChannel::R]);
        assert_eq!(color.g, color[RGBAChannel::G]);
        assert_eq!(color.b, color[RGBAChannel::B]);
        assert_eq!(color.a, color[RGBAChannel::A]);
    }

    #[test]
    fn rgba_default_ut() {
        assert_eq!(RGBA::from_values(0, 0, 0, 255), RGBA::default());
    }

    #[test]
    fn rgba_new_ut() {
        assert_eq!(RGBA::new(), RGBA::default());
    }
}
