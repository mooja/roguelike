use rltk;
use specs::{prelude::*, Component, storage::BTreeStorage};
use specs_derive::*;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
#[storage(BTreeStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: rltk::RGB,
    pub bg: rltk::RGB,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool
}


#[derive(Component)]
#[storage(VecStorage)]
pub struct Monster {}