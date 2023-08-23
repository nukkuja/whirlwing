pub trait Window {
    type Error: std::fmt::Display;

    fn init(
        title: &str,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn destroy(&mut self) -> Result<(), Self::Error>;

    fn draw_background(&mut self);
}
