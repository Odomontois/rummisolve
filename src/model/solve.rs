use std::array::from_fn;

use super::{all_combos, Tile, TileSet};

#[allow(unused)]
pub fn solve(pool: TileSet) -> Option<Vec<TileSet>> {
    let mut solve = Solve::new(pool);
    solve.solve()?;
    Some(solve.accum)
}

#[derive(Debug)]
struct Solve {
    accum: Vec<TileSet>,
    pool: TileSet,
    combos_by_tile: [Vec<TileSet>; Tile::SIZE],
    avail_combos: [u16; Tile::SIZE],
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

        let accum = Vec::new();
        let avail_combos = from_fn(|i| combos_by_tile[i].len() as u16);

        Self {
            accum,
            pool,
            avail_combos,
            combos_by_tile,
        }
    }

    fn check_fail(&self) -> Option<()> {
        (!self.tiles_by_combo_count[0].is_empty()).then_some(())
    }

    fn normalize(&mut self) -> Option<()> {
        self.check_fail()?;
        while let Some(c) = self.tiles_by_combo_count[1].tiles().next() {
            let combo = self.combos_by_tile[c.code() as usize][0];
            self.apply_combo(combo)?;
            self.check_fail()?;
        }

        None
    }

    fn solve(&mut self) -> Option<()> {
        self.normalize()?;

        None
    }

    fn apply_combo(&mut self, combo: TileSet) -> Option<()> {
		self.pool -= combo;
		self.accum.push(combo);
        todo!()
    }
}

#[test]
#[ignore]
fn check() {
    println!("{}", std::mem::size_of::<Option<Vec<TileSet>>>());
}
