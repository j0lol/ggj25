use core::cell::RefCell;

use crate::{screen, tile, Matrix2D, State, Tile, Tiles, BUBBLE};
use agb::{display::object::{OamManaged, Object}, fixnum::Vector2D};
use alloc::{rc::{Rc, Weak}, vec::Vec};

pub struct Bubble<'oac> {
    // id : u8,
    pub motion : Vector2D<i16>,
    pub contents : Object<'oac>,
    pub picked_up : Option<Weak<Object<'oac>>>,
}

impl<'oac> Bubble<'oac> {

    pub fn new(position: Vector2D<i32>, oam: &'oac OamManaged) -> Self {
        let mut new_bubble = oam.object_sprite(BUBBLE.sprite(0));
        new_bubble
            .set_position(position)
            .show();

        Bubble {
            contents: new_bubble,
            motion: Vector2D { x: 0, y: 0 },
            picked_up: None
        }
    }

    pub fn push(&mut self, direction: Vector2D<i16>) -> () {
        if !(direction == Vector2D::<i16>{x:0,y:1} || direction == Vector2D::<i16> {x:0,y:-1} || direction == Vector2D::<i16> {x:1,y:0} || direction == Vector2D::<i16> {x:-1,y:0}) {
            unreachable!();
        }
        self.motion = direction * 16;
        agb::println!("bubel in motion {} {}", self.motion.x, self.motion.y);
    }

    /// => Some(self) = bubble should still exist
    /// => None = bubble popped. maybe have a little pop animation?
    pub fn step(&mut self, block: Option<Rc<RefCell<Object>>>, tiles : &Tiles) -> bool {
        let next_pos = tile(self.contents.position()) + self.motion.change_base();

        if let Some(block) = block {
            // Take ownership
        } else {

        }

        todo!();

//         if let Some((index, _block)) = boxes.iter().enumerate().find(|(_, o)| o.position() == screen(next_pos)) {
//             match &mut self.picked_up {
//                 None => {
//                     // let block = &mut boxes[index];
//                     // block.hide();
//                     // let blockref = alloc::rc::Rc::new(block);
//                    // let blockref = alloc::rc::Rc::<Object<'_>>::downgrade(&blockref);
//
//                     //self.picked_up = Some(blockref);
//                     Some(self)
//                 }
//                 Some(weak) => {
//                     //weak.upgrade().expect("not strong").show();
//                     None
//                 }
//             }
//         } else if tiles.get(tile(next_pos.change_base()).x as usize, tile(next_pos.change_base()).y as usize).unwrap() == &Tile::Wall {
//             let right = Vector2D {x: self.motion.y, y: self.motion.x * -1};
//             let left = Vector2D {x: self.motion.y * -1, y: self.motion.x};
//             match (tiles.get(tile(self.contents.position() + right.change_base()).x as usize, tile(self.contents.position() + right.change_base()).y as usize).unwrap(),
//                 (tiles.get(tile(self.contents.position() + left.change_base()).x as usize, tile(self.contents.position() + left.change_base()).y as usize).unwrap())) {
//                 (&Tile::Wall, &Tile::Wall) => {
//                     if let Some(weak) = self.picked_up {
//                         //weak.upgrade().expect("not strong").show();
//                         //boxes.push(self.picked_up.take().unwrap());
//                     }
//                     None
//                 }
//                 (&Tile::Wall, _) => {
//                     self.motion = left;
//                     Some(self)
//                 }
//                 (_, &Tile::Wall) => {
//                     self.motion = right;
//                     Some(self)
//                 }
//                 _ => Some(self)
//             }
//         } else {
//             self.contents.set_position(self.contents.position() + (self.motion * 16).change_base());
//             if let Some(ref mut picked) = self.picked_up {
//                 // oh no!
//                 // picked.set_position(self.contents.position());
//             }
//             Some(self)
//         }
    }
}


