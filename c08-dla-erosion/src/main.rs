use backend::*;

struct RoomBuilder {}

impl RoomBuilder {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl MapGen for RoomBuilder {
    fn setup(&mut self) {}

    fn build(&mut self) -> Vec<(Map, String)> {
        let mut frames = Vec::new();
        let mut rooms = Vec::<(Rect, usize)>::new();
        let mut rng = RandomNumberGenerator::new();
        let mut map = Map::new();

        let mut room_counter = 0;
        for _ in 0..20 {
            let room = Rect::with_size(
                rng.range(1, WIDTH as i32 - 10),
                rng.range(1, HEIGHT as i32 - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for (r, _) in rooms.iter() {
                let mut r_grow = r.clone();
                r_grow.x1 -= 1;
                r_grow.y1 -= 1;
                r_grow.x2 += 1;
                r_grow.y2 += 1;
                if r_grow.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    map.set(p, to_cp437('#'), iteration_color(room_counter));
                });
                rooms.push((room, room_counter));
                room_counter += 1;
            } else {
                let mut discard = map.clone();
                room.for_each(|p| {
                    discard.set(p, to_cp437('!'), iteration_color(666));
                });
            }
        }

        // Sort it
        rooms.sort_by(|a, b| a.0.x1.cmp(&b.0.x1));
        map.clear_default();
        for (room, iteration) in rooms.iter() {
            room.for_each(|p| {
                map.set(p, to_cp437('#'), iteration_color(*iteration));
            });
        }

        // Corridors
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].0.center();
            let new = room.0.center();

            if rng.range(0, 2) == 1 {
                apply_horizontal_tunnel(prev.x, new.x, prev.y, &mut map);
                apply_vertical_tunnel(prev.y, new.y, new.x, &mut map);
            } else {
                apply_vertical_tunnel(prev.y, new.y, prev.x, &mut map);
                apply_horizontal_tunnel(prev.x, new.x, new.y, &mut map);
            }
        }
        map.tiles
            .iter_mut()
            .filter(|(tt, _)| *tt == to_cp437('#'))
            .for_each(|(_, col)| *col = RGB::named(YELLOW));
        frames.push((
            map.clone(),
            "Start with a traditional set of rooms".to_string(),
        ));

        // Erode with DLA
        for i in 0..500 {
            map.tiles
                .iter_mut()
                .filter(|t| t.1 == RGB::named(RED))
                .for_each(|t| t.1 = RGB::named(GREEN));

            let open_tiles: Vec<Point> = map
                .tiles
                .iter()
                .enumerate()
                .filter(|(_i, (tt, _col))| *tt == to_cp437('#'))
                .map(|(i, (_tt, _col))| Point::new(i % WIDTH, i / WIDTH))
                .collect();

            let mut digger = *rng.random_slice_entry(&open_tiles).unwrap();
            let mut digger_idx = mapidx(digger.x, digger.y);
            while map.tiles[digger_idx].0 == to_cp437('#') {
                let stagger_direction = rng.roll_dice(1, 4);
                match stagger_direction {
                    1 => {
                        if digger.x > 2 {
                            digger.x -= 1;
                        }
                    }
                    2 => {
                        if digger.x < WIDTH as i32 - 2 {
                            digger.x += 1;
                        }
                    }
                    3 => {
                        if digger.y > 2 {
                            digger.y -= 1;
                        }
                    }
                    _ => {
                        if digger.y < HEIGHT as i32 - 2 {
                            digger.y += 1;
                        }
                    }
                }
                digger_idx = mapidx(digger.x, digger.y);
            }
            map.set(digger, to_cp437('#'), RGB::named(RED));
            frames.push((map.clone(), format!("Iteration {}", i)));
        }

        frames
    }
}

fn apply_horizontal_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    use std::cmp::{max, min};
    for x in min(x1, x2)..=max(x1, x2) {
        if let Some(idx) = map.try_idx(Point::new(x, y)) {
            map.tiles[idx as usize] = (to_cp437('#'), RGB::named(PURPLE));
        }
    }
}

fn apply_vertical_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    use std::cmp::{max, min};
    for y in min(y1, y2)..=max(y1, y2) {
        if let Some(idx) = map.try_idx(Point::new(x, y)) {
            map.tiles[idx as usize] = (to_cp437('#'), RGB::named(PURPLE));
        }
    }
}

fn main() -> BError {
    run(RoomBuilder::new())
}
