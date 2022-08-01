#![allow(unused)]

use rltk::{GameState, Rltk, RGB};
use specs::{Join, WorldExt};
use specs_derive;

mod components;
mod map;
mod systems;
use components::*;
struct State {
    map: map::GameMap,
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

        let renderables = self.ecs.read_storage::<Renderable>();
        let positions = self.ecs.read_storage::<Position>();

        let mut drawn = vec![0u8; 80 * 50];
        for (pos, render) in (&positions, &renderables).join() {
            let drawn_idx = (pos.y * 80 + pos.x) as usize;
            if render.level > drawn[drawn_idx] {
                drawn[drawn_idx] = render.level;
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }

        self.dirty = false;
    }
}

impl State {
    fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        let mut positions = self.ecs.write_storage::<Position>();
        let players = self.ecs.read_storage::<Player>();
        let walls = self.ecs.read_storage::<Wall>();

        let (mut dest_x, mut dest_y) = (0, 0);

        for (_player, pos) in (&players, &mut positions).join() {
            dest_x = (pos.x + delta_x).rem_euclid(self.map.w as i32);
            dest_y = (pos.y + delta_y).rem_euclid(self.map.h as i32);
        }

        for (_wall, pos) in (&walls, &positions).join() {
            if pos.x == dest_x && pos.y == dest_y {
                return;
            }
        }

        for (_player, pos) in (&players, &mut positions).join() {
            pos.x = dest_x;
            pos.y = dest_y;
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
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    use specs::prelude::*;

    let mut term = RltkBuilder::simple80x50().with_title("Roguelike").build()?;
    let mut gs = State {
        map: map::GameMap::new_with_rooms(),
        ecs: specs::World::new(),
        dirty: true,
    };

    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Floor>();
    gs.ecs.register::<components::Wall>();

    // create the player entity
    gs.ecs
        .create_entity()
        .with(components::Player)
        .with(components::Position {
            x: gs.map.rooms[0].center_pos().x,
            y: gs.map.rooms[0].center_pos().y,
        })
        .with(components::Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            level: 5,
        })
        .build();

    // create the map tile entities
    for (idx, tile) in gs.map.tiles.iter().enumerate() {
        let x = idx % gs.map.w;
        let y = idx / gs.map.w;

        let mut builder = gs.ecs.create_entity().with(components::Position {
            x: x as i32,
            y: y as i32,
        });

        match tile {
            map::TileType::Floor => {
                builder = builder
                    .with(components::Floor)
                    .with(components::Renderable {
                        glyph: rltk::to_cp437('.'),
                        fg: RGB::named(rltk::TEAL),
                        bg: RGB::named(rltk::BLACK),
                        level: 1,
                    });
            }

            map::TileType::Wall => {
                builder = builder.with(components::Wall).with(components::Renderable {
                    glyph: rltk::to_cp437('#'),
                    fg: RGB::named(rltk::TEAL),
                    bg: RGB::named(rltk::BLACK),
                    level: 1,
                });
            }
        }

        builder.build();
    }

    rltk::main_loop(term, gs)
}
