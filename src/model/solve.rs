use std::array::from_fn;

use super::{all_combos, Tile, TileSet};

#[allow(unused)]
pub fn solve(pool: TileSet) -> Option<Vec<TileSet>> {
    let mut solve = Solve::new(pool);
    solve.solve().then(|| solve.accum)
}

struct Solve {
    accum: Vec<TileSet>,
    pool: TileSet,
    combos_by_tile: [Vec<TileSet>; Tile::SIZE],
    avail_combos: [u16; Tile::SIZE],
    tiles_by_combo_count: Vec<TileSet>,
}

impl Solve {
    pub fn new(pool: TileSet) -> Self {
        let mut combos_by_tile = from_fn(|_| Vec::new());
        for combo in all_combos().filter(|c| c <= &pool) {
            for tile in combo.unique_tiles() {
                let i = tile.code() as usize;
                combos_by_tile[i].push(combo);
            }
        }
        let max_combo_count = combos_by_tile.iter().map(|v| v.len()).max().unwrap_or(0);
        let mut tiles_by_combo_count = vec![TileSet::default(); max_combo_count + 1];
        for (i, combos) in combos_by_tile.iter().enumerate() {
            let tile = Tile::from_code(i as u64).unwrap();
            tiles_by_combo_count[combos.len()] += tile;
        }

        let accum = Vec::new();
        let avail_combos = from_fn(|i| combos_by_tile[i].len() as u16);

        Self {
            accum,
            pool,
            avail_combos,
            combos_by_tile,
            tiles_by_combo_count,
        }
    }

    pub fn solve(&mut self) -> bool {
		if !self.tiles_by_combo_count[0].is_empty() {
			return false
		}
        false
    }
}

#[test]
#[ignore]
fn check() {
    println!("{}", std::mem::size_of::<Option<Vec<TileSet>>>());
}
