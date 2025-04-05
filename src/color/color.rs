pub trait Color {
    fn new() -> Self;
    fn complementary(&self) -> Self;
    fn to_hex(&self) -> String;
    fn from_hex(hex: &str) -> Self;
}