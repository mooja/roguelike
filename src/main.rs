#![allow(unused)]

use rltk::{Algorithm2D, BaseMap, GameState, Rltk, RGB};
use specs::{Join, RunNow, WorldExt};
use specs_derive;

mod components;
mod map;
mod systems;
use components::*;
use map::{GameMap, TileType};

struct State {
    ecs: specs::World,
    dirty: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.process_kb_input(ctx);

        if !self.dirty {
            return;
        }

        ctx.cls();

        // render map
        let (mut x, mut y) = (0, 0);
        let m = self.ecs.fetch::<GameMap>();
        for tile in &m.tiles {
            match tile {
                TileType::Wall => ctx.set(
                    x,
                    y,
                    RGB::named(rltk::TEAL),
                    RGB::named(rltk::BLACK),
                    rltk::to_cp437('#'),
                ),
                TileType::Floor => ctx.set(
                    x,
                    y,
                    RGB::named(rltk::TEAL),
                    RGB::named(rltk::BLACK),
                    rltk::to_cp437('.'),
                ),
            }

            x += 1;
            if x == m.w {
                x = 0;
                y += 1;
            }
        }

        // render player
        let players = self.ecs.read_storage::<Player>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (_player, pos, render) in (&players, &positions, &renderables).join() {
            ctx.set(
                pos.x,
                pos.y,
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('@'),
            );
        }

        self.dirty = false;
    }
}

impl State {
    fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        let mut positions = self.ecs.write_storage::<Position>();
        let players = self.ecs.read_storage::<Player>();
        for (p, pos) in (&players, &mut positions).join() {
            let m = self.ecs.fetch::<GameMap>();
            let (dest_x, dest_y) = (pos.x + delta_x, pos.y + delta_y);
            let dest_tile = &m.tiles[m.xy_idx(dest_x, dest_y)];
            match dest_tile {
                &TileType::Floor => {
                    pos.x = dest_x;
                    pos.y = dest_y;
                    self.dirty = true;
                }
                _ => {}
            }
        }
    }

    fn process_kb_input(&mut self, term: &mut Rltk) {
        use rltk::VirtualKeyCode;
        match term.key {
            None => {}
            Some(key) => {
                self.dirty = true;

                match key {
                    VirtualKeyCode::Up | VirtualKeyCode::K => self.move_player(0, -1),
                    VirtualKeyCode::Down | VirtualKeyCode::J => self.move_player(0, 1),
                    VirtualKeyCode::Left | VirtualKeyCode::H => self.move_player(-1, 0),
                    VirtualKeyCode::Right | VirtualKeyCode::L => self.move_player(1, 0),
                    _ => {}
                };
            }
        }
    }

    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    use specs::prelude::*;

    let mut term = RltkBuilder::simple80x50().with_title("Roguelike").build()?;
    let mut gs = State {
        ecs: specs::World::new(),
        dirty: true,
    };

    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Viewshed>();

    let mut m = map::GameMap::new_with_rooms();

    // create the player entity
    gs.ecs
        .create_entity()
        .with(components::Player)
        .with(components::Position {
            x: m.rooms[0].center_pos().x,
            y: m.rooms[0].center_pos().y,
        })
        .with(components::Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            level: 5,
        })
        .with(components::Viewshed {
            visible_tiles: vec![],
            range: 8,
        })
        .build();

    gs.ecs.insert(m);

    rltk::main_loop(term, gs)
}
