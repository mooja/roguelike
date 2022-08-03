use std::fmt::Write;

use specs::{Join, ReadStorage, System, SystemData, WriteStorage, ReadExpect};
use super::components::{Player, Position, Viewshed};
use rltk::{field_of_view, Point, GameState};

// pub struct VisibilitySystem ;

// impl<'a> System<'a> for VisibilitySystem {
//     type SystemData = (
//         WriteStorage<'a, Viewshed>,
//         ReadStorage<'a, Position>,
//         ReadStorage<'a, Player>,
//     );

//     fn run(&mut self, (mut viewshed, positions, players): Self::SystemData) {
//         for (mut viewshed, pos, player) in (&mut viewshed, &positions, &players).join() {
//             viewshed.visible_tiles.clear();
//             viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range);
//         }
//     }
// }
