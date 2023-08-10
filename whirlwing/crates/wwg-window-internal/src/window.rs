pub trait Window {
    type Error: std::fmt::Display;

    fn make_current(&self) -> Result<(), Self::Error>;
    fn destroy(self) -> Result<(), Self::Error>;
}
