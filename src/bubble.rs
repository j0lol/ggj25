use crate::{screen, tile, Matrix2D, State, Tile, Tiles};
use agb::{display::object::Object, fixnum::Vector2D};

pub struct Bubble<'oac> {
    // id : u8,
    pub motion : Vector2D<i16>,
    pub contents : Object<'oac>,
    pub picked_up : Option<Object<'oac>>,
}

impl<'oac> Bubble<'oac> {
    pub fn push(&mut self, direction: Vector2D<i16>) -> () {
        if !(direction == Vector2D::<i16>{x:0,y:1} || direction == Vector2D::<i16> {x:0,y:-1} || direction == Vector2D::<i16> {x:1,y:0} || direction == Vector2D::<i16> {x:-1,y:0}) {
            unreachable!();
        }
        self.motion = direction * 16;
    }

    pub fn step(mut self, state : &'oac mut State<'oac>, tiles : Tiles) -> Option<Bubble<'oac>> {
        let next_pos = tile(self.contents.position()) + self.motion.change_base();
        if let Some((index, _block)) = state.boxes.iter().enumerate().find(|(_, o)| o.position() == screen(next_pos)) {
            match &mut self.picked_up {
                None => {
                    self.picked_up = Some(state.boxes.swap_remove(index)); Some(self)
                }
                Some(_) => {
                    state.boxes.push(self.picked_up.take().unwrap()); None
                }
            }
        } else if tiles.get(tile(next_pos.change_base()).x as usize, tile(next_pos.change_base()).y as usize).unwrap() == &Tile::Wall {
            let right = Vector2D {x: self.motion.y, y: self.motion.x * -1};
            let left = Vector2D {x: self.motion.y * -1, y: self.motion.x};
            match (tiles.get(tile(self.contents.position() + right.change_base()).x as usize, tile(self.contents.position() + right.change_base()).y as usize).unwrap(),
                (tiles.get(tile(self.contents.position() + left.change_base()).x as usize, tile(self.contents.position() + left.change_base()).y as usize).unwrap())) {
                (&Tile::Wall, &Tile::Wall) => {
                    if let Some(_) = self.picked_up {
                        state.boxes.push(self.picked_up.take().unwrap()); 
                    }
                    None
                }
                (&Tile::Wall, _) => {
                    self.motion = left;
                    Some(self)
                }
                (_, &Tile::Wall) => {
                    self.motion = right;
                    Some(self)
                }
                _ => Some(self)
            }
        } else {None}
    }
}


