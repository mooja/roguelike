use std::fmt::Write;

use super::components::{Player, Position, Viewshed};
use super::map;
use rltk::{field_of_view, Algorithm2D, GameState, Point};
use specs::{Join, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage};

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, map::GameMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut viewshed, pos, mut map) = data;
        for (mut viewshed, pos) in (&mut viewshed, &pos).join() {
            if !viewshed.dirty {
                break;
            }

            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.w as i32 && p.y >= 0 && p.y < map.h as i32);

            for vt in &viewshed.visible_tiles {
                let idx = map.point2d_to_index(*vt);
                map.revealed_tiles[idx] = true;
            }
        }
    }
}
