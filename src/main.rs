#![allow(unused)]

use rltk::{GameState, Rltk};
use specs;
use specs_derive;

mod components;

struct State {
    map: map::GameMap,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.map.draw(ctx);
    }
}

#[allow(unused, dead_code)]
mod map {
    pub struct Pos {
        x: i32,
        y: i32,
    }

    #[derive(Clone, Copy)]
    pub enum TileType {
        Wall,
        Floor,
    }

    pub struct GameMap {
        pub w: u32,
        pub h: u32,
        pub tiles: Vec<TileType>,
    }

    impl GameMap {
        pub fn new_simple() -> GameMap {
            let mut m = GameMap {
                w: 80,
                h: 50,
                tiles: vec![TileType::Floor; 80 * 50],
            };

            for x in 0..m.w {
                let idx = m.xy_idx(x, 0);
                m.tiles[idx] = TileType::Wall;
            }

            m
        }

        pub fn xy_idx(&self, x: u32, y: u32) -> usize {
            (x as u32 + self.w * y as u32) as usize
        }

        pub fn draw(&self, ctx: &mut rltk::Rltk) {
            let mut x = 0;
            let mut y = 0;
            for t in &self.tiles {
                match t {
                    TileType::Floor => ctx.set(x, y, rltk::TEAL, rltk::BLACK, rltk::to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, rltk::TEAL, rltk::BLACK, rltk::to_cp437('#')),
                };

                x += 1;
                if x == self.w {
                    x = 0;
                    y += 1;
                }
            }
        }

        pub fn generate_rooms(&mut self) {
            unimplemented!()
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50().with_title("Roguelike").build()?;
    let gs = State {
        map: map::GameMap::new_simple(),
    };
    rltk::main_loop(context, gs)
}
