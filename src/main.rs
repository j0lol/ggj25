#![no_std]
#![no_main]

// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::*;
use agb::input::Button;

const TILE_SIZE: u16 = 16_u16; // px

static GRAPHICS: &display::object::Graphics = include_aseprite!("gfx/spritesheet.aseprite");

// thank you HackSussex for the player name
static PLAYER: &display::object::Tag = GRAPHICS.tags().get("Garlick");
static BLOCK: &display::object::Tag = GRAPHICS.tags().get("Block");
static BUBBLE: &display::object::Tag = GRAPHICS.tags().get("Bubble");

struct GameObject<'a> {
    oam_object: display::object::Object<'a>
}

extern crate alloc;
use alloc::vec::Vec;
struct Matrix2D<T>{
    width: usize,
    height: usize,
    internal: alloc::vec::Vec<T>
}

impl<T> Matrix2D<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.internal.get(x * self.width + y)
    }
}

type Tiles = Matrix2D<bool>;



struct Player {
    tilepos: fixnum::Vector2D<i16>,
    movement_intent: fixnum::Vector2D<i16>,
    move_lock: u16,
}

fn direction_dispatch(input: &input::ButtonController) -> Option<fixnum::Vector2D<i16>> {

    if input.x_tri() == input::Tri::Zero && input.y_tri() == input::Tri::Zero {
        return None;
    }
    return Some(fixnum::Vector2D::new(input.x_tri() as i16, input.y_tri() as i16));
}

impl Player {

    fn new(x: i16, y: i16) -> Player {
        Player {
            tilepos: fixnum::Vector2D::new(x, y),
            movement_intent: fixnum::Vector2D::new(0, 0),
            move_lock: 0,
        }
    }

    fn input(&mut self, input: &input::ButtonController, oammanaged: &display::object::OamManaged) {
        if self.move_lock == 0 {
            if let Some(intent) = direction_dispatch(input) {
                self.movement_intent = intent;
                self.tilepos += self.movement_intent;
                self.move_lock = 16;
            }
        } else if self.move_lock > 0 {
            // tween? TODO
            self.move_lock -= 1;
        } else {
            panic!("NEGATIVE LOCK");
        }

        if (input.is_just_pressed(Button::A)) {
            let mut new_bubble = oammanaged.object_sprite(BUBBLE.sprite(0));
            new_bubble.set_x((self.tilepos.x * TILE_SIZE as i16) as u16).set_y((self.tilepos.y * TILE_SIZE as i16) as u16).show();
        }
    }


    fn update(&self, oam: &mut display::object::Object) {
        oam.set_position(self.tilepos.change_base() * TILE_SIZE as i32);
    }
}

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {

    let object: display::object::OamManaged = gba.display.object.get_managed();

    let mut player = object.object_sprite(PLAYER.sprite(0));
    let mut input = agb::input::ButtonController::new();

    let mut pl = Player::new(3, 3);

    player.set_x(50).set_y(50).show();

    loop {
        pl.input(&input, &object);
        pl.update(&mut player);

        agb::display::busy_wait_for_vblank();
        object.commit();
        input.update();
    }
}
