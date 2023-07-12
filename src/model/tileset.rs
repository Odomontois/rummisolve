use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{cmp::Ordering, iter::from_fn};

use super::Tile;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct TileSet {
    once: u64,
    twice: u64,
}

impl TileSet {
    pub fn cointains(self, tile: Tile) -> bool {
        self.once & 1 << tile.code() != 0
    }

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
    pub fn unique_tiles(self) -> impl Iterator<Item = Tile> {
        let mut bitmap = self.once;
        from_fn(move || {
            let tz = bitmap.trailing_zeros() as u64;
            let tile = Tile::from_code(tz)?;
            bitmap &= !(1 << tz);
            Some(tile)
        })
    }

    pub fn is_empty(self) -> bool {
        self.once == 0 && self.twice == 0
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

impl Add for TileSet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let once = self.once | rhs.once;
        let twice = self.twice | rhs.twice | (self.once & rhs.once);
        Self { once, twice }
    }
}

impl Sub for TileSet {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let twice = self.twice & (!rhs.once);
        let once = (self.once & (!rhs.once)) | (self.twice & (!rhs.twice));
        Self { once, twice }
    }
}

impl SubAssign for TileSet {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl Add<Tile> for TileSet {
    type Output = Self;

    fn add(self, rhs: Tile) -> Self::Output {
        self.add(rhs)
    }
}

impl AddAssign<Tile> for TileSet {
    fn add_assign(&mut self, rhs: Tile) {
        *self = self.add(rhs);
    }
}

impl Sub<Tile> for TileSet {
    type Output = Self;

    fn sub(self, rhs: Tile) -> Self::Output {
        self.remove(rhs)
    }
}

impl SubAssign<Tile> for TileSet {
    fn sub_assign(&mut self, rhs: Tile) {
        *self = self.remove(rhs);
    }
}

impl Debug for TileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

impl IntoIterator for TileSet {
    type Item = Tile;

    type IntoIter = Tiles;

    fn into_iter(self) -> Self::IntoIter {
        Tiles(self)
    }
}

pub struct Tiles(TileSet);

impl Iterator for Tiles {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        let tz = self.0.once.trailing_zeros() as u64;
        let tile = Tile::from_code(tz)?;
        let bit = 1 << tz;
        if (self.0.twice & bit) != 0 {
            self.0.twice ^= bit;
        } else {
            self.0.once ^= bit;
        }
        Some(tile)
    }
}
