#![no_std]
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use core::cell::RefCell;

use agb::display::tiled::TiledMap;
use agb::input::Button;
use agb::*;
use agb_tracker::{include_midi, Track};
use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use alloc::vec::Vec;
use bubble::Bubble;
use display::object::{OamManaged, Object};
use display::{
    palette16::Palette16,
    tiled::{RegularBackgroundSize, TileFormat},
    Priority,
};
use fixnum::Vector2D;
use sound::mixer::SoundChannel;

mod bubble;
mod level;
mod player;

const TILE_SIZE: u16 = 16_u16; // px

static GRAPHICS: &display::object::Graphics = include_aseprite!("gfx/spritesheet.aseprite");

agb::include_background_gfx!(tiles, tiles => "gfx/tileset.aseprite");
agb::include_background_gfx!(title, title => "gfx/title.aseprite");

// thank you HackSussex for the player name
static PLAYER: &display::object::Tag = GRAPHICS.tags().get("Garlick");
static BLOCK: &display::object::Tag = GRAPHICS.tags().get("Block");
static BUBBLE: &display::object::Tag = GRAPHICS.tags().get("Bubble");

struct GameObject<'a> {
    oam_object: display::object::Object<'a>,
}

#[derive(Clone, Debug)]
struct Matrix2D<T> {
    width: usize,
    height: usize,
    internal: alloc::vec::Vec<T>,
}

impl<T> Matrix2D<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.internal.get(y * self.width + x)
    }
    pub fn from_vec(vec: Vec<T>, width: usize, height: usize) -> Matrix2D<T> {
        Matrix2D::<T> {
            width,
            height,
            internal: vec,
        }
    }
}

static bgm: &[u8] = include_wav!("bgm/awa.wav");

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Target,
    Block,
    FulfilledTarget,
    PlayerSpawn,
}

// Converts 16x to 4 8x8 tiles
pub fn tile_indexer(t: usize, tilemap_width: usize) -> (usize, usize, usize, usize) {
    let num = t * 2;
    (
        num,
        num + 1,
        tilemap_width * 2 + num,
        tilemap_width * 2 + num + 1,
    )
}

type Tiles = Matrix2D<Tile>;

// Tilespace to Screenspace, named functionally (badly)
fn screen(v2: fixnum::Vector2D<i16>) -> fixnum::Vector2D<i32> {
    (v2 * TILE_SIZE as i16).change_base()
}
fn tile(v2: fixnum::Vector2D<i32>) -> fixnum::Vector2D<i16> {
    let Vector2D { x, y } = v2;
    Vector2D::new(x as i16, y as i16) / TILE_SIZE as i16
}

struct State<'a> {
    bubbles: Vec<Rc<RefCell<Bubble<'a>>>>,
    boxes: Vec<Rc<RefCell<Object<'a>>>>,
}

impl<'a> State<'a> {
    fn new() -> Self {
        State::<'a> {
            boxes: Vec::new(),
            bubbles: Vec::new(),
        }
    }
}

#[agb::entry]
fn titlescreen(mut gba: agb::Gba) -> ! {
    let mut input = agb::input::ButtonController::new();

    {
        let (gfx, mut vram) = gba.display.video.tiled0();

        let mut map = gfx.background(
            agb::display::Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        vram.set_background_palettes(title::PALETTES);

        map.fill_with(&mut vram, &title::title);

        map.commit(&mut vram);
        map.set_visible(true);
    }

    loop {
        if (input.is_just_pressed(Button::A | Button::START)) {
            let mut level = 0;
            loop {
                let n = main(gba, level);
                gba = n.0;
                let win = n.1;
                level += win as usize;
            }
        }

        agb::display::busy_wait_for_vblank();
        input.update();
    }
}

fn main(mut gba: agb::Gba, level_num: usize) -> (agb::Gba, bool) {
    let object: display::object::OamManaged = gba.display.object.get_managed();

    let mut player = object.object_sprite(PLAYER.sprite(0));

    let mut input = agb::input::ButtonController::new();

    let level = level::Level::new(level_num);

    let mut mixer = gba.mixer.mixer(sound::mixer::Frequency::Hz18157);
    let mut bgm_chan = SoundChannel::new_high_priority(bgm);
    bgm_chan.should_loop();
    mixer.enable();
    mixer.play_sound(bgm_chan).unwrap();

    let (px, py) = level::player_spawn(&level.tiles);

    let mut pl = player::Player::new(px as _, py as _);
    player
        .set_x(px as u16 * TILE_SIZE)
        .set_y(py as u16 * TILE_SIZE)
        .show();

    let mut state = State::new();
    state.boxes = level.make_boxes(&object);

    let (gfx, mut vram) = gba.display.video.tiled0();
    let tileset = &tiles::tiles.tiles;

    vram.set_background_palettes(tiles::PALETTES);

    let mut bg = gfx.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        tileset.format(),
    );

    level.draw(&mut vram, &mut bg);
    bg.commit(&mut vram);
    bg.set_visible(true);

    loop {
        pl.input(&input, &object, &mut state, &level.tiles);
        pl.update(&mut player);

        if pl.stepped {
            let mut to_remove = Vec::new();
            for (index, bubble) in state.bubbles.iter_mut().enumerate() {
                // find box intersecting with bubble

                let next_pos = tile(bubble.borrow().contents.position())
                    + bubble.borrow().motion.change_base();

                let block = if let Some((index, _block)) = state
                    .boxes
                    .iter()
                    .enumerate()
                    .find(|(_, o)| o.borrow().position() == screen(next_pos))
                {
                    Some(state.boxes.swap_remove(index))
                } else {
                    None
                };

                let (block, exists) = bubble.borrow_mut().step(block, &level.tiles);

                if let Some(block) = block {
                    state.boxes.push(block);
                }
                if !exists {
                    agb::println!("NOTEXISTS {}", index);
                    to_remove.push(index);
                }
            }
            to_remove.sort();
            to_remove.reverse();
            to_remove.iter().for_each(|index| {
                agb::println!("{}", index);
                let bubel = state.bubbles.swap_remove(*index);
                if let Some(ref dropped) = bubel.clone().borrow().picked_up {
                    state.boxes.push(dropped.clone());
                }
                ()
            });
        }

        let wintile = level.win_tile();

        let win = state
            .boxes
            .iter()
            .all(|o| wintile.contains(&tile(o.borrow_mut().position())) )

            && state.bubbles.is_empty();

        mixer.frame();

        agb::display::busy_wait_for_vblank();
        object.commit();
        input.update();

        // yuo win!;
        if win {
            drop(player);
            drop(state);
            drop(bg);
            return (gba, true);
        }
        if input.is_just_pressed(Button::START) {
            drop(player);
            drop(state);
            drop(bg);
            return (gba, false);
        }
    }
}
