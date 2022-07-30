
#[derive(PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct GameMap {
    pub w: usize,
    pub h: usize,
    pub tiles: Vec<TileType>,
}

impl GameMap {
    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        (x + self.w * y)
    }

    fn tile_entry(&mut self, x: usize, y: usize) -> &'_ mut TileType {
        let idx = self.xy_idx(x, y);
        &mut self.tiles[idx]
    }

    fn draw_borders(&mut self) {
        for x in 0..self.w {
            *self.tile_entry(x, 0) = TileType::Wall;
            *self.tile_entry(x, self.h - 1) = TileType::Wall;
        }

        for y in 0..self.h {
            *self.tile_entry(0, y) = TileType::Wall;
            *self.tile_entry(self.w - 1, y) = TileType::Wall;
        }
    }

    pub fn new_simple() -> GameMap {
        let mut m = GameMap {
            w: 80,
            h: 50,
            tiles: vec![TileType::Floor; 80 * 50],
        };

        m.draw_borders();

        let mut rng = rltk::RandomNumberGenerator::new();
        for idx in 0..m.tiles.len() {
            if rng.range(0, 9) == 0 {
                m.tiles[idx] = TileType::Wall;
            }
        }

        m
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
