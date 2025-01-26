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

use core::cell::RefCell;

use crate::{screen, tile_indexer, Matrix2D, Tile, Tiles, BLOCK};
use agb::{
    display::{object::{OamManaged, Object}, tiled::{MapLoan, RegularMap, Tiled0, VRamManager}},
    fixnum::Vector2D,
};
use alloc::{rc::Rc, string::String, vec::Vec};

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
        }
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

    pub fn make_boxes<'obj>(&self, object: &'obj OamManaged) -> Vec<Rc<RefCell<Object<'obj>>>> {
        let mut boxes = Vec::new();

        for (n, t) in self.tiles.internal.iter().enumerate() {
            if let Tile::Block = t {
                let (x, y) = (n % self.tiles.width, n / self.tiles.width);

                let mut new_box = object.object_sprite(BLOCK.sprite(0));
                new_box
                    .set_position(screen(Vector2D::new(x as i16, y as i16)))
                    .show();
                boxes.push(Rc::new(RefCell::new(new_box)));
            }
        }

        boxes
    }

    pub fn draw<'m>(&self, vram: &mut VRamManager, bg: &mut MapLoan<'m, RegularMap>) {
        let tileset = &crate::tiles::tiles.tiles;

        for y in 0..self.tiles.height as u16 {
            for x in 0..self.tiles.width as u16 {
                let tile = self.tiles.get(x as usize, y as usize).unwrap();
                let (t1, t2, t3, t4) = tile_indexer(*tile as usize, 6);

                bg.set_tile(
                    vram,
                    (x * 2, y * 2),
                    tileset,
                    crate::tiles::tiles.tile_settings[t1],
                );
                bg.set_tile(
                    vram,
                    (x * 2 + 1, y * 2),
                    tileset,
                    crate::tiles::tiles.tile_settings[t2],
                );
                bg.set_tile(
                    vram,
                    (x * 2, y * 2 + 1),
                    tileset,
                    crate::tiles::tiles.tile_settings[t3],
                );
                bg.set_tile(
                    vram,
                    (x * 2 + 1, y * 2 + 1),
                    tileset,
                    crate::tiles::tiles.tile_settings[t4],
                );
            }
        }
    }
}