use std::collections::HashMap;

use crate::model::{all_combos, TileSet};

use super::Tile;

pub fn debug_info() -> Vec<[String; 2]> {
    [
        item("Combination count", all_combos().count()),
        item("Assignment count", all_combos().flatten().count()),
        item(
            "Jokerless combinations",
            all_combos()
                .filter(|c: &TileSet| !c.cointains(Tile::Joker))
                .count(),
        ),
        item(
            "Jokerless assignment count",
            all_combos()
                .filter(|c: &TileSet| !c.cointains(Tile::Joker))
                .flatten()
                .count(),
        ),
        item("Maximum combos per tile (No Joker)", {
            let mut count = HashMap::new();
            for tile in all_combos().flatten().filter(|x| x != &Tile::Joker) {
                *count.entry(tile).or_insert(0) += 1;
            }
            count.values().copied().max().unwrap_or(0)
        }),
        item("Minimum combos per tile (No Joker)", {
            let mut count = HashMap::new();
            for tile in all_combos().flatten().filter(|x| x != &Tile::Joker) {
                *count.entry(tile).or_insert(0) += 1;
            }
            count.values().copied().min().unwrap_or(0)
        }),
    ]
    .into()
}

fn item<A: ToString, B: ToString>(a: A, b: B) -> [String; 2] {
    [a.to_string(), b.to_string()]
}
