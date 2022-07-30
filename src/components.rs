use specs::{prelude::*, Component};
use specs_derive::*;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
#[storage(VecStorage)]
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
    pub level: u8
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Wall;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Floor;
