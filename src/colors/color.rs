use super::{RGB, RGBA};

pub trait Color {
    /// Create new `Color`
    ///
    fn new() -> Self;

    /// Generate a hex color `String`.
    fn to_hex(&self) -> String;

    /// Create a `Color` from a hex `String`
    ///
    fn from_hex(hex: &str) -> Self;

    /// Create a `Color` from `RGB` values
    ///
    fn from_rgb(rgb: RGB) -> Self;

    /// Create a `Color` from `RGBA` values
    ///
    fn from_rgba(rgba: RGBA) -> Self;

    // Utils

    /// Returns a random `Color`.
    ///
    fn random() -> Self;

    /// Returns a `Color` value comlementary to own value.
    ///
    fn complementary(&self) -> Self;
}
