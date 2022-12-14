use std::fmt::Write;

use super::components::{Monster, Player, Position, Viewshed};
use super::map;
use rltk::{field_of_view, Algorithm2D, GameState, Point};
use specs::{
    Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage,
};

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, map::GameMap>,
        ReadStorage<'a, Player>,
        Entities<'a>,
        ReadExpect<'a, super::SleepState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut viewshed, pos, mut map, player, e, ss) = data;
        for (e, mut viewshed, pos) in (&e, &mut viewshed, &pos).join() {
            if !viewshed.dirty {
                break;
            }

            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.w as i32 && p.y >= 0 && p.y < map.h as i32);

            if let Some(p) = player.get(e) {
                for vt in &viewshed.visible_tiles {
                    let idx = vt.to_index(map.w);
                    map.revealed_tiles[idx] = true;
                }
            }
        }
    }
}

pub struct MonsterAI;
impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            vs,
            pos,
            monster,
            player) = data;

        for (pl, pl_pos) in (&player, &pos).join() {
            for (vs, pos, monster) in (&vs, &pos, &monster).join() {
                if vs
                    .visible_tiles
                    .contains(&rltk::Point::new(pl_pos.x, pl_pos.y))
                {
                    println!("A monster sees you!")
                }
            }
        }
    }
}