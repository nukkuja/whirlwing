pub trait WindowingContext {
    type Window: crate::Window;
    type Error: std::fmt::Display;

    fn init() -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn create_window(
        &self,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self::Window, Self::Error>;

    fn destroy_context(self) -> Result<(), Self::Error>;
}
