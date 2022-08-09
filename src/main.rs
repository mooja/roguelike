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
        self.run_systems();

        if self.dirty {
            self.draw_map(ctx);
            self.dirty = false;
        }
    }
}

impl State {
    fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        let mut positions = self.ecs.write_storage::<Position>();
        let players = self.ecs.read_storage::<Player>();
        let mut vs = self.ecs.write_storage::<Viewshed>();
        for (p, pos, vs) in (&players, &mut positions, &mut vs).join() {
            let m = self.ecs.fetch::<GameMap>();
            let (dest_x, dest_y) = (pos.x + delta_x, pos.y + delta_y);
            let dest_tile = &m.tiles[m.xy_idx(dest_x, dest_y)];
            match dest_tile {
                &TileType::Floor => {
                    pos.x = dest_x;
                    pos.y = dest_y;
                    self.dirty = true;
                    vs.dirty = true;
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
        let mut vis_system = systems::VisibilitySystem {};
        vis_system.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn draw_map(&self, ctx: &mut Rltk) {
        ctx.cls();

        let m = self.ecs.fetch::<GameMap>();
        let vs = self.ecs.read_storage::<Viewshed>();
        let pl = self.ecs.read_storage::<Player>();
        let pos = self.ecs.read_storage::<Position>();
        let ren = self.ecs.read_storage::<Renderable>();
        let mon = self.ecs.read_storage::<Monster>();

        // render map tiles
        for (_pl, vs, player_pos) in (&pl, &vs, &pos).join() {
            for (idx, tile) in m.tiles.iter().enumerate() {
                if !m.revealed_tiles[idx] {
                    continue;
                }

                let render_pos = m.index_to_point2d(idx);
                let mut fg_color = if vs.visible_tiles.contains(&render_pos) {
                    RGB::named(rltk::GREEN)
                } else {
                    RGB::named(rltk::GRAY)
                };
                let bg_color = RGB::named(rltk::BLACK);
                let mut glyph = match tile {
                    TileType::Floor => rltk::to_cp437('.'),
                    TileType::Wall => rltk::to_cp437('#'),
                };

                ctx.set(render_pos.x, render_pos.y, fg_color, bg_color, glyph);
            }
        }

        // render player
        for (pl, pos, ren) in (&pl, &pos, &ren).join() {
            ctx.set(pos.x, pos.y, ren.fg, ren.bg, ren.glyph);
        }

        // render monsters
        for (pl, vs) in (&pl, &vs).join() {
            for (mon, pos, ren) in (&mon, &pos, &ren).join() {
                if vs.visible_tiles.contains(&rltk::Point::new(pos.x, pos.y)) {
                    ctx.set(pos.x, pos.y, ren.fg, ren.bg, ren.glyph);
                }
            }
        }
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
    gs.ecs.register::<components::Monster>();

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
        })
        .with(components::Viewshed {
            visible_tiles: vec![],
            range: 8,
            dirty: true,
        })
        .build();

    for r in &m.rooms {
        gs.ecs
            .create_entity()
            .with(components::Monster {})
            .with(components::Position {
                x: r.center_pos().x,
                y: r.center_pos().y,
            })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    gs.ecs.insert(m);

    rltk::main_loop(term, gs)
}
