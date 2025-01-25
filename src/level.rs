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

use agb::{display::object::{OamManaged, Object}, fixnum::Vector2D};
use alloc::{string::String, vec::Vec};
use crate::{screen, Matrix2D, Tile, Tiles, BLOCK};

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

pub struct Level {
    pub tiles: Tiles,
}

impl Level {
    pub fn new() -> Self {
        let tiles = level_parse(LEVEL);
        Level { tiles }
    }

    pub fn make_boxes<'obj>(&self, object: &'obj OamManaged) -> Vec<Object<'obj>> {

        let mut boxes = Vec::new();

        for (n, t) in self.tiles.internal.iter().enumerate() {
            if let Tile::Block = t {
                let (x,y) = (n % self.tiles.width, n / self.tiles.width);

                let mut new_box = object.object_sprite(BLOCK.sprite(0));
                new_box
                    .set_position(screen(Vector2D::new(x as i16, y as i16)))
                    .show();
                boxes.push(new_box);
            }
        }

        boxes
    }
}