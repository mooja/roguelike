use specs::{prelude::*, Component};
use specs_derive::*;


#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    x: i32,
    y: i32
}