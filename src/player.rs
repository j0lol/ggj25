use agb::{
    display, fixnum,
    input::{self, Button},
};

use crate::{screen, Bubble, State, Tile, Tiles, BUBBLE};
use fixnum::Vector2D;

pub struct Player {
    pub tilepos: fixnum::Vector2D<i16>,
    pub movement_intent: fixnum::Vector2D<i16>,
    pub move_lock: u16,
}

const fn ddispatch(b: Button) -> Vector2D<i16> {
    match b {
        Button::LEFT => Vector2D::new(-1, 0),
        Button::RIGHT => Vector2D::new(1, 0),
        Button::UP => Vector2D::new(0, -1),
        Button::DOWN => Vector2D::new(0, 1),
        _ => panic!("oh you..."),
    }
}

// I'm so sorry. This code is horrible but kind of has to be.
// We could theoretically make this a lot nicer by just grabbing
// the internal bit string of the Buttons and handling it ourselves.
// Not worth it in a JAM though!
pub fn direction_dispatch(
    input: &input::ButtonController,
    move_lock: &mut u16,
) -> Option<fixnum::Vector2D<i16>> {
    let dpad_buttons: Button = Button::LEFT | Button::RIGHT | Button::UP | Button::DOWN;

    if input.is_just_pressed(dpad_buttons) {
        if input.is_just_pressed(Button::LEFT) {
            return Some(ddispatch(Button::LEFT));
        }
        if input.is_just_pressed(Button::RIGHT) {
            return Some(ddispatch(Button::RIGHT));
        }
        if input.is_just_pressed(Button::UP) {
            return Some(ddispatch(Button::UP));
        }
        if input.is_just_pressed(Button::DOWN) {
            return Some(ddispatch(Button::DOWN));
        }
    } else if *move_lock == 0 {
        if input.is_pressed(Button::LEFT) {
            return Some(ddispatch(Button::LEFT));
        }
        if input.is_pressed(Button::RIGHT) {
            return Some(ddispatch(Button::RIGHT));
        }
        if input.is_pressed(Button::UP) {
            return Some(ddispatch(Button::UP));
        }
        if input.is_pressed(Button::DOWN) {
            return Some(ddispatch(Button::DOWN));
        }
    } else {
        *move_lock -= 1;
    }
    return None;
}

impl Player {
    pub fn new(x: i16, y: i16) -> Player {
        Player {
            tilepos: fixnum::Vector2D::new(x, y),
            movement_intent: fixnum::Vector2D::new(0, 0),
            move_lock: 0,
        }
    }

    pub fn input<'inp, 'oam: 'inp>(
        &mut self,
        input: &input::ButtonController,
        oammanaged: &'oam display::object::OamManaged<'oam>,
        state: &'inp mut State<'oam>,
        level: &Tiles,
    ) {
        // Movement
        if let Some(intent) = direction_dispatch(input, &mut self.move_lock) {
            self.movement_intent = intent;

            let future_movement = self.tilepos + self.movement_intent;
            let collide = Tile::Wall
                == *(level
                    .get(future_movement.x as usize, future_movement.y as usize)
                    .unwrap())
                || state
                    .boxes
                    .iter()
                    .any(|o| o.position() == screen(future_movement));
            if collide {
                // agb::println!("nah");
            } else {
                if let Some(bubble) = state
                    .bubbles
                    .iter_mut()
                    .find(|o| o.contents.position() == screen(future_movement))
                {
                    agb::println!("bubel {} {}", self.movement_intent.x, self.movement_intent.y);
                    bubble.push(self.movement_intent);
                }
                self.tilepos += self.movement_intent;
                self.move_lock = 12;
            }
        }

        // Bubble spawner
        if input.is_just_pressed(Button::A) {
            let mut new_bubble = oammanaged.object_sprite(BUBBLE.sprite(0));
            new_bubble
                .set_position(screen(self.tilepos + self.movement_intent))
                .show();
            let bubble = Bubble {
                contents: new_bubble,
                motion: Vector2D { x: 0, y: 0 },
                picked_up: None
            };
            state.bubbles.push(bubble);
        }
    }

    pub fn update(&self, oam: &mut display::object::Object) {
        oam.set_position(screen(self.tilepos));
    }
}
