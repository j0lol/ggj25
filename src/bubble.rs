use crate::{State};
use agb::{display::object::Object, fixnum::Vector2D};
use alloc::rc::Rc;

pub struct Bubble<'oac> {
    pub motion : Vector2D<i16>,
    pub contents : Object<'oac>,
}

impl<'aoc> Bubble<'aoc> {
    pub fn push(&mut self, direction: Vector2D<i16>) -> () {
        if (direction == Vector2D::<i16>{x:0,y:1} || direction == Vector2D::<i16> {x:0,y:-1} || direction == Vector2D::<i16> {x:1,y:0} || direction == Vector2D::<i16> {x:-1,y:0}) {
            panic!();
        }
        self.motion = direction;
    }

    pub fn step(self, state : &mut State) -> () {

    }
}
