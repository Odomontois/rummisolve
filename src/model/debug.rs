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
        )
    ]
    .into()
}

fn item<A: ToString, B: ToString>(a: A, b: B) -> [String; 2] {
    [a.to_string(), b.to_string()]
}
