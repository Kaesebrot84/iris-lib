use std::fmt;

use super::RGB;

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
        RGBA {
            r: r,
            g: g,
            b: b,
            a,
        }
    }
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ R: {}, G: {}, B: {}, A: {} }}",
            self.r, self.g, self.b, self.a
        )
    }
}

/// Indexing implementation for the `RGBA` struct using `RGBAChannel` as index.
///
/// # Examples
///
/// ```
/// use iris_lib::color::{RGBA, RGBAChannel};
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

impl From<RGB> for RGBA {
    fn from(rgb: RGB) -> Self {
        RGBA::from_values(rgb.r, rgb.g, rgb.b, 255)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgba_index_ut() {
        let color = RGBA {
            r: 1,
            g: 2,
            b: 3,
            a: 4,
        };
        assert_eq!(1, color[RGBAChannel::R]);
        assert_eq!(2, color[RGBAChannel::G]);
        assert_eq!(3, color[RGBAChannel::B]);
        assert_eq!(4, color[RGBAChannel::A]);
    }
}
