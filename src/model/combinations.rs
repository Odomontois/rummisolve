use super::{Color, Tile, TileSet, Value};

#[allow(unused)]
pub fn all_combos() -> impl Iterator<Item = TileSet> {
    jokerless_combos()
}

fn jokerless_combos() -> impl Iterator<Item = TileSet> {
    repeated_combos().chain(sequence_combos())
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
