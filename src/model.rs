#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Black,
    Blue,
}

use Color::*;
impl Color {
    pub fn name(self) -> &'static str {
        match self {
            Red => "red",
            Green => "green",
            Black => "black",
            Blue => "blue",
        }
    }

    pub fn all() -> impl Iterator<Item = Color> {
        [Red, Green, Black, Blue].iter().copied()
    }
}
