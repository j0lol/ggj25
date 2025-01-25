use agb::{
    display, fixnum,
    input::{self, Button},
};

use crate::{screen, State, Tile, Tiles, BUBBLE};

pub struct Player {
    pub tilepos: fixnum::Vector2D<i16>,
    pub movement_intent: fixnum::Vector2D<i16>,
    pub move_lock: u16,
}

pub fn direction_dispatch(input: &input::ButtonController) -> Option<fixnum::Vector2D<i16>> {
    if input.x_tri() == input::Tri::Zero && input.y_tri() == input::Tri::Zero {
        return None;
    }
    return Some(fixnum::Vector2D::new(
        input.x_tri() as i16,
        input.y_tri() as i16,
    ));
}

impl Player {
    pub fn new(x: i16, y: i16) -> Player {
        Player {
            tilepos: fixnum::Vector2D::new(x, y),
            movement_intent: fixnum::Vector2D::new(0, 0),
            move_lock: 0,
        }
    }

    pub fn input<'oam>(
        &mut self,
        input: &input::ButtonController,
        oammanaged: &'oam display::object::OamManaged<'oam>,
        state: &mut State<'oam>,
        level: &Tiles,
    ) {
        // Movement
        if self.move_lock == 0 {
            if let Some(intent) = direction_dispatch(input) {
                self.movement_intent = intent;

                if Tile::Wall == *(
                    level
                        .get(
                            (self.tilepos + self.movement_intent).x as usize,
                            (self.tilepos + self.movement_intent).y as usize,
                        )
                        .unwrap()
                ) || state.boxes.iter().any(|o| o.position() == screen(self.tilepos + self.movement_intent)) {
                    agb::println!("nah");
                } else {
                    self.tilepos += self.movement_intent;
                    self.move_lock = 16;
                }
            }
        } else if self.move_lock > 0 {
            // tween? TODO
            self.move_lock -= 1;
        } else {
            panic!("NEGATIVE LOCK");
        }

        // Bubble spawner
        if input.is_just_pressed(Button::A) {
            let mut new_bubble = oammanaged.object_sprite(BUBBLE.sprite(0));
            new_bubble
                .set_position(screen(self.tilepos + self.movement_intent))
                .show();
            state.bubbles.push(new_bubble);
        }
    }

    pub fn update(&self, oam: &mut display::object::Object) {
        oam.set_position(screen(self.tilepos));
    }
}
