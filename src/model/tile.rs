
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum Color {
    #[default]
    Red,
    Green,
    Black,
    Blue,
}

use std::fmt::{Debug, Display};

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

    fn code(self) -> u64 {
        match self {
            Red => 0,
            Green => 1,
            Black => 2,
            Blue => 3,
        }
    }

    fn from_code(code: u64) -> Option<Self> {
        Some(match code {
            0 => Red,
            1 => Green,
            2 => Black,
            3 => Blue,
            _ => return None,
        })
    }

    pub fn all() -> impl Iterator<Item = Color> {
        [Red, Green, Black, Blue].iter().copied()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Value(u8);

impl Value {
    pub fn all() -> impl Iterator<Item = Value> {
        (1..=13).map(Value)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub fn from_code(code: u64) -> Option<Self> {
        if code < 1 || code > 13 {
            return None;
        }
        Some(Self(code as u8))
    }
}

impl Default for Value {
    fn default() -> Self {
        Value(1)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Tile {
    Normal { color: Color, value: Value },
    Joker,
}

impl Tile {
    const JOKER_CODE: u64 = 52;
    // pub const SIZE: usize = 53;

    pub fn code(self) -> u64 {
        match self {
            Tile::Normal { color, value } => {
                let color = color.code();
                let value = value.value() as u64;
                value * 4 + color
            }
            Tile::Joker => Self::JOKER_CODE,
        }
    }
    pub fn from_code(code: u64) -> Option<Self> {
        if code == Self::JOKER_CODE {
            return Some(Self::Joker);
        }
        let color = Color::from_code(code % 4)?;
        let value = Value::from_code(code / 4)?;
        Some(Self::Normal { color, value })
    }

    #[allow(unused)]
    pub fn all() -> impl Iterator<Item = Tile> {
        Color::all()
            .flat_map(move |color| Value::all().map(move |value| Self::Normal { color, value }))
            .chain([Self::Joker])
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Normal { color, value } => {
                let letter = match color {
                    Color::Red => 'R',
                    Color::Green => 'G',
                    Color::Black => 'B',
                    Color::Blue => 'U',
                };

                write!(f, "{letter}{value}")
            }
            Tile::Joker => write!(f, "J"),
        }
    }
}