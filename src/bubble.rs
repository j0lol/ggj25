use crate::Block;
use agb::fixnum::Vector2D;
use alloc::rc::Rc;

pub struct Bubble {
    pub position : Vector2D<i16>,
    pub motion : Vector2D<i16>,
    pub contents : Rc<Block>,
}


