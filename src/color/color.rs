use super::{RGB, RGBA};

pub trait Color {
    // TODO
    fn new() -> Self;
    // TODO
    fn to_hex(&self) -> String;
    
    // TODO
    fn from_hex(hex: &str) -> Self;
    // TODO
    fn from_rgb(rgb: RGB) -> Self;
    // TODO
    fn from_rgba(rgba: RGBA) -> Self;

    // Utils

    // TODO
    fn complementary(&self) -> Self;
}