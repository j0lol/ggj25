use core::cell::RefCell;

use crate::{screen, tile, Matrix2D, State, Tile, Tiles, BUBBLE};
use agb::{
    display::object::{OamManaged, Object},
    fixnum::Vector2D,
};
use alloc::{
    rc::{Rc, Weak},
    vec::Vec,
};

pub struct Bubble<'oac> {
    // id : u8,
    pub motion: Vector2D<i16>,
    pub contents: Object<'oac>,
    pub picked_up: Option<Rc<RefCell<Object<'oac>>>>,
}

impl<'oac> Bubble<'oac> {
    pub fn new(position: Vector2D<i32>, oam: &'oac OamManaged) -> Self {
        let mut new_bubble = oam.object_sprite(BUBBLE.sprite(0));
        new_bubble.set_position(position).show();

        Bubble {
            contents: new_bubble,
            motion: Vector2D { x: 0, y: 0 },
            picked_up: None,
        }
    }

    pub fn push(&mut self, direction: Vector2D<i16>) -> () {
        if !(direction == Vector2D::<i16> { x: 0, y: 1 }
            || direction == Vector2D::<i16> { x: 0, y: -1 }
            || direction == Vector2D::<i16> { x: 1, y: 0 }
            || direction == Vector2D::<i16> { x: -1, y: 0 })
        {
            unreachable!();
        }
        self.motion = direction;
        agb::println!("bubel in motion {} {}", self.motion.x, self.motion.y);
    }

    /// => Some(self) = bubble should still exist
    /// => None = bubble popped. maybe have a little pop animation?
    pub fn step(&mut self, block: Option<Rc<RefCell<Object<'oac>>>>, tiles: &Tiles) -> bool {
        let next_pos = tile(self.contents.position()) + self.motion;

        if let Some(block) = block {
            // Take ownership
            match &mut self.picked_up {
                &mut Some(_) => {
                    false
                }
                &mut None => {
                    self.picked_up = Some(block);
                    true
                }
            }
        } else if tiles.get(next_pos.x as usize, next_pos.y as usize).unwrap() == &Tile::Wall {
            let right = Vector2D {
                x: self.motion.y,
                y: self.motion.x * -1,
            };
            let left = Vector2D {
                x: self.motion.y * -1,
                y: self.motion.x,
            };
            let tilepos = tile(self.contents.position());
            match (
                tiles
                    .get(
                        (tilepos + right).x as usize,
                        (tilepos + right).y as usize,
                    )
                    .unwrap(),
                (tiles
                    .get(
                        (tilepos + left).x as usize,
                        (tilepos + left).y as usize,
                    )
                    .unwrap()),
            ) {
                (&Tile::Wall, &Tile::Wall) => {
                    false
                }
                (&Tile::Wall, _) => {
                    self.motion = left;
                    true
                }
                (_, &Tile::Wall) => {
                    self.motion = right;
                    true
                }
                _ => {
                    false
                }
            }
        } else {
            self.contents.set_position(screen(next_pos));
            self.picked_up.as_mut().inspect(|bx| {bx.borrow_mut().set_position(screen(next_pos)); ()});
            true
        }
    }
}


