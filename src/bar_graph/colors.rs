#[derive(Debug, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl Default for Color {
    fn default() -> Self {
        Self(255, 255, 255)
    }
}
