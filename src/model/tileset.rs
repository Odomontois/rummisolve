use std::{cmp::Ordering, iter::from_fn};

use super::Tile;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct TileSet {
    once: u64,
    twice: u64,
}

impl TileSet {
    pub fn amount(self, tile: Tile) -> u8 {
        let code_bit = 1 << tile.code();
        if self.once & code_bit == 0 {
            return 0;
        }
        if self.twice & code_bit == 0 {
            return 1;
        }
        2
    }

    pub fn add(self, tile: Tile) -> Self {
        let code_bit = 1 << tile.code();
        if self.once & code_bit == 0 {
            Self {
                once: self.once | code_bit,
                ..self
            }
        } else {
            Self {
                twice: self.twice | code_bit,
                ..self
            }
        }
    }

    pub fn remove(self, tile: Tile) -> Self {
        let code_bit = 1 << tile.code();
        if self.twice & code_bit != 0 {
            Self {
                twice: self.twice & !code_bit,
                ..self
            }
        } else {
            Self {
                once: self.once & !code_bit,
                ..self
            }
        }
    }
    fn unique_tiles(self) -> impl Iterator<Item = Tile> {
        let mut bitmap = self.once;
        from_fn(move || {
            let tz = bitmap.trailing_zeros() as u64;
            let tile = Tile::from_code(tz)?;
            bitmap &= !(1 << tz);
            Some(tile)
        })
    }

    pub fn tiles(self) -> impl Iterator<Item = Tile> {
        self.unique_tiles()
            .flat_map(move |t| (0..self.amount(t)).map(move |_| t))
    }
}

impl FromIterator<Tile> for TileSet {
    fn from_iter<T: IntoIterator<Item = Tile>>(iter: T) -> Self {
        iter.into_iter()
            .fold(Self::default(), |set, tile| set.add(tile))
    }
}

impl PartialOrd for TileSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        let common_once = self.once & other.once;
        let common_twice = self.twice & other.twice;
        if common_once == self.once && common_twice == self.twice {
            Some(Ordering::Less)
        } else if common_once == other.once && common_twice == other.twice {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
