use crate::{screen, State, tile};
use agb::{display::object::Object, fixnum::Vector2D};

pub struct Bubble<'oac> {
    pub motion : Vector2D<i16>,
    pub contents : Object<'oac>,
}

impl<'aoc> Bubble<'aoc> {
    pub fn push(&mut self, direction: Vector2D<i16>) -> () {
        if !(direction == Vector2D::<i16>{x:0,y:1} || direction == Vector2D::<i16> {x:0,y:-1} || direction == Vector2D::<i16> {x:1,y:0} || direction == Vector2D::<i16> {x:-1,y:0}) {
            panic!();
        }
        self.motion = direction;
    }

    pub fn step(self, state : &mut State) -> () {
        let next_pos = tile(self.contents.position()) + self.motion.change_base();
        if let Some(block) = state.boxes.iter().find(|o| o.position() == screen(next_pos)) {}
    }
}
