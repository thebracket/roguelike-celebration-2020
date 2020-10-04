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
        for _ in 0..50 {
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

        // Start/end
        let desired_start = Point::new(0, HEIGHT / 2);
        let start = map
            .tiles
            .iter()
            .enumerate()
            .map(|(i, (tt, _col))| (i, *tt))
            .filter(|(_i, tt)| *tt == to_cp437('#'))
            .map(|(i, _tt)| {
                (
                    i,
                    DistanceAlg::Pythagoras
                        .distance2d(desired_start, Point::new(i % WIDTH, i / WIDTH)),
                )
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        // Preferred end
        let desired_end = Point::new(WIDTH - 1, HEIGHT / 2);
        let end = map
            .tiles
            .iter()
            .enumerate()
            .map(|(i, (tt, _col))| (i, *tt))
            .filter(|(_i, tt)| *tt == to_cp437('#'))
            .map(|(i, _tt)| {
                (
                    i,
                    DistanceAlg::Pythagoras
                        .distance2d(desired_end, Point::new(i % WIDTH, i / WIDTH)),
                )
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        map.set(
            Point::new(start % WIDTH, start / WIDTH),
            to_cp437('@'),
            RGB::named(GOLD),
        );
        map.set(
            Point::new(end % WIDTH, end / WIDTH),
            to_cp437('>'),
            RGB::named(GOLD),
        );
        map.tiles.iter_mut().for_each(|t| if t.0==to_cp437('#') {
            t.1 = RGB::named(GREEN);
        });
        frames.push((map.clone(), "Rooms with Start/End".to_string()));

        // Find the path
        let path = a_star_search(start, end, &map);
        for r in rooms.iter() {
            let mut hit = false;
            path.steps.iter().for_each(|idx| {
                let mut room = r.clone();
                room.0.y2 += 1;
                let p = Point::new(idx % WIDTH, idx / WIDTH);
                if room.0.point_in_rect(p) {
                    hit = true;
                }
            });
            if hit {
                r.0.for_each(|p| {
                    if map.tiles[mapidx(p.x, p.y)].0 != to_cp437('@') && map.tiles[mapidx(p.x, p.y)].0 != to_cp437('>') {
                        map.set(p, to_cp437('#'), RGB::named(YELLOW));
                    }
                });
            } else {
                r.0.for_each(|p| {
                    if map.tiles[mapidx(p.x, p.y)].0 != to_cp437('@') && map.tiles[mapidx(p.x, p.y)].0 != to_cp437('>') {
                        map.set(p, to_cp437('#'), RGB::named(GRAY));
                    }
                });
            }
        }
        frames.push((map.clone(), "Important Rooms Highlighted".to_string()));

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
