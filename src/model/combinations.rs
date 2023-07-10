use std::{collections::HashSet, hash::Hash};

use super::{Color, Tile, TileSet, Value};

#[allow(unused)]
pub fn all_combos() -> impl Iterator<Item = TileSet> {
    dedup(jokerless_combos().flat_map(jokerized))
}

fn dedup<A: Eq + Hash + Copy + 'static>(xs: impl Iterator<Item = A>) -> impl Iterator<Item = A> {
    xs.scan(HashSet::new(), |s, e| {
        let new = !s.contains(&e);
        s.insert(e);
        Some(new.then(|| e))
    })
    .flatten()
}

fn jokerless_combos() -> impl Iterator<Item = TileSet> {
    repeated_combos().chain(sequence_combos())
}

fn jokerized(combo: TileSet) -> impl Iterator<Item = TileSet> {
    [combo]
        .into_iter()
        .chain(single_joker(combo))
        .chain(double_joker(combo))
}

fn single_joker(combo: TileSet) -> impl Iterator<Item = TileSet> {
    combo
        .tiles()
        .map(move |tile| combo.remove(tile).add(Tile::Joker))
}

fn double_joker(combo: TileSet) -> impl Iterator<Item = TileSet> {
    combo
        .tiles()
        .flat_map(move |t1| {
            combo
                .tiles()
                .filter_map(move |t2| (t1 < t2).then(|| (t1, t2)))
        })
        .map(move |(t1, t2)| {
            combo
                .remove(t1)
                .remove(t2)
                .add(Tile::Joker)
                .add(Tile::Joker)
        })
}

fn repeated_combos() -> impl Iterator<Item = TileSet> {
    Value::all().flat_map(move |value| {
        Color::all()
            .map(Some)
            .chain([None])
            .map(move |exclude| repeated(value, exclude))
    })
}

fn repeated(value: Value, exclude: Option<Color>) -> TileSet {
    Color::all()
        .filter(|&color| (Some(color) != exclude))
        .map(|color| Tile::Normal { color, value })
        .collect()
}

fn sequence_combos() -> impl Iterator<Item = TileSet> {
    Color::all().flat_map(color_sequences)
}

fn color_sequences(color: Color) -> impl Iterator<Item = TileSet> {
    (1..11).flat_map(move |start| {
        (start + 2..=13).map(move |end| {
            (start..=end)
                .filter_map(Value::from_code)
                .map(move |value| Tile::Normal { color, value })
                .collect()
        })
    })
}
