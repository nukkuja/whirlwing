pub trait Window {
    type Error: std::fmt::Display;

    fn make_current(&self) -> Result<(), Self::Error>;
    fn draw_background(&self, r: f32, g: f32, b: f32, a: f32);
    fn destroy(self) -> Result<(), Self::Error>;
}
