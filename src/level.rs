pub const LEVEL: &'static str = r#"
XXXXXXXXXXXXXXX
X    tX       X
X  XXXX       X
X             X
X             X
X    b        X
X      p      X
X             X
X             X
XXXXXXXXXXXXXXX
"#;

use alloc::{string::String, vec::Vec};
use crate::{Matrix2D, Tile, Tiles};

// const w/o debug
fn tile_dispatch(c: char) -> Tile {
    match c {
        ' ' => Tile::Empty,
        'X' => Tile::Wall,
        'p' => Tile::PlayerSpawn,
        't' => Tile::Target,
        'b' => Tile::Block,
        c => {
            panic!("{:?}", c);
        },
    }
}

pub fn level_parse(level: &str) -> Tiles {
    let lines: Vec<_> = level.trim().lines().collect();

    let width = lines[0].trim().len();
    let height = lines.len();

    let level: String = level.trim().replace('\n', "");
    let level: Vec<_> = level.chars().map(tile_dispatch).collect();

    Matrix2D::<Tile>::from_vec(level, width, height)
}

pub fn player_spawn(map: &Tiles) -> (usize, usize) {
    for (n, t) in map.internal.iter().enumerate() {
        if let Tile::PlayerSpawn = t {
            return (n % map.width, n / map.width);
        }
    }

    panic!("you didnt put a spawn!");
}