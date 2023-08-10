pub trait Window {
    type Error;

    fn make_current(&self) -> Result<(), Self::Error>;
    fn destroy(self) -> Result<(), Self::Error>;
}
