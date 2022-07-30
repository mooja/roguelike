#[derive(PartialEq, Clone, Copy)]
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

    pub fn new_with_rooms() -> GameMap {
        const NROOMS: usize = 15;
        const ROOM_MIN_SIZE: usize = 7;
        const ROOM_MAX_SIZE: usize = 15;

        let mut m = GameMap {
            w: 80,
            h: 50,
            tiles: vec![TileType::Wall; 80 * 50],
        };

        use rltk::RandomNumberGenerator;
        let mut rng = RandomNumberGenerator::new();
        let mut gen_attempts = 0;
        let mut rooms = vec![];

        while rooms.len() < NROOMS as usize && gen_attempts < 1000 {
            let room_candidate = Room {
                tl: Pos {
                    x: rng.range(1, (m.w - ROOM_MIN_SIZE) as i32),
                    y: rng.range(1, (m.h - ROOM_MIN_SIZE) as i32),
                },
                h: rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE),
                w: rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE),
            };

            gen_attempts += 1;

            if room_candidate.br().x as usize >= m.w || room_candidate.br().y as usize >= m.h {
                continue;
            }

            let mut overlaps = false;
            for r in &rooms {
                if room_candidate.overlaps(r) {
                    overlaps = true;
                    break;
                }
            }

            if overlaps {
                continue;
            }

            rooms.push(room_candidate);
        }

        for r in &rooms {
            for x in r.tl.x..r.br().x {
                for y in r.tl.y..r.br().y {
                    *m.tile_entry(x as usize, y as usize) = TileType::Floor;
                }
            }
        }

        for i in 0..rooms.len() - 1 {
            let r1 = &rooms[i];
            let r2 = &rooms[i + 1];

            for p in r1.connecting_path(&r2) {
                *m.tile_entry(p.x as usize, p.y as usize) = TileType::Floor;
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

pub struct Room {
    pub tl: Pos,
    pub w: usize,
    pub h: usize,
}

impl Room {
    pub fn overlaps(&self, other: &Room) -> bool {
        if self.tl == other.tl {
            return true;
        }

        // reorder rooms by by top left x point
        let mut rooms = [self, other];
        rooms.sort_by(|&r1, &r2| r1.tl.x.cmp(&r2.tl.x));
        let [r1, r2] = rooms;

        let mut x_axis_overlap = false;
        if r2.tl.x <= r1.tl.x + (r1.w as i32) {
            x_axis_overlap = true;
        }

        if !x_axis_overlap {
            return false;
        }

        // reorder rooms by by top left y point
        let mut rooms = [self, other];
        rooms.sort_by(|&r1, &r2| r1.tl.y.cmp(&r2.tl.y));
        let [r1, r2] = rooms;

        let mut y_axis_overlap = false;
        if r2.tl.y <= r1.tl.y + (r1.h as i32) {
            y_axis_overlap = true;
        }

        x_axis_overlap && y_axis_overlap
    }

    pub fn connecting_path(&self, other: &Room) -> Vec<Pos> {
        let mut p = self.center_pos();
        let dest = other.center_pos();
        let mut path = vec![];
        let x_step = if (dest.x - p.x) > 0 { 1 } else { -1 };
        while p.x != dest.x {
            p.x += x_step;
            path.push(p);
        }

        let y_step = if (dest.y - p.y) > 0 { 1 } else { -1 };
        while p.y != dest.y {
            p.y += y_step;
            path.push(p);
        }

        path
    }

    pub fn center_pos(&self) -> Pos {
        Pos {
            x: self.tl.x + (self.w / 2) as i32,
            y: self.tl.y + (self.h / 2) as i32,
        }
    }

    pub fn br(&self) -> Pos {
        Pos {
            x: self.tl.x + self.w as i32,
            y: self.tl.y + self.h as i32,
        }
    }
}
