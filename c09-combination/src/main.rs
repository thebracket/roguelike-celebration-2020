use backend::*;

struct RoomBuilder {
    rects: Vec<Rect>,
    rooms: Vec<Rect>,
}

impl RoomBuilder {
    fn new() -> Box<Self> {
        Box::new(Self {
            rects: Vec::new(),
            rooms: Vec::new(),
        })
    }
}

impl MapGen for RoomBuilder {
    fn setup(&mut self) {}

    fn build(&mut self) -> Vec<(Map, String)> {
        let mut frames = Vec::new();

        // Make the first room
        self.rects
            .push(Rect::with_size(2, 2, WIDTH as i32 - 5, HEIGHT as i32 - 5));

        // Divide
        let first_room = self.rects[0];
        self.add_subrects(first_room);

        let mut map = Map::new();
        for (_i, room) in self.rects.iter().enumerate() {
            room.for_each(|p| {
                map.set(p, to_cp437('#'), RGB::named(YELLOW));
            });
        }

        let mut rng = RandomNumberGenerator::new();
        let mut map = Map::new();
        let mut n_rooms = 0;
        while n_rooms < 240 {
            let rect = self.get_random_rect(&mut rng);
            let candidate = self.get_random_sub_rect(rect, &mut rng);
            if self.is_possible(candidate, &map) {
                candidate.for_each(|p| {
                    map.set(p, to_cp437('#'), RGB::named(YELLOW));
                });
                self.rooms.push(candidate);
                self.add_subrects(rect);
            }
            n_rooms += 1;
        }

        // Sort it
        self.rooms.sort_by(|a, b| a.x1.cmp(&b.x1));
        map.clear_default();
        for (_iteration, room) in self.rooms.iter().enumerate() {
            room.for_each(|p| {
                map.set(p, to_cp437('#'), RGB::named(YELLOW));
            });
        }

        // Corridors
        for (i, room) in self.rooms.iter().enumerate().skip(1) {
            let prev = self.rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                apply_horizontal_tunnel(prev.x, new.x, prev.y, &mut map);
                apply_vertical_tunnel(prev.y, new.y, new.x, &mut map);
            } else {
                apply_vertical_tunnel(prev.y, new.y, prev.x, &mut map);
                apply_horizontal_tunnel(prev.x, new.x, new.y, &mut map);
            }
        }
        frames.push((map.clone(), "Make some sub-division rooms".to_string()));

        // Store it
        let bsp = map.clone();

        // Make a CA map
        map.tiles.iter_mut().for_each(|(t, c)| {
            let roll = rng.range(0, 100);
            if roll < 55 {
                *t = to_cp437('.');
                *c = RGB::named(DARK_GRAY);
            } else {
                *t = to_cp437('#');
                *c = RGB::named(YELLOW);
            }
        });

        for _ in 0..4 {
            iterate(&mut map);
        }
        frames.push((map.clone(), "Make a Cellular Automata Map".to_string()));
        let ca = map.clone();

        // Combine the two
        map = Map::new();
        let center_x = WIDTH as i32 / 2;
        for y in 0..HEIGHT as i32 {
            for x in 0..center_x {
                let idx = mapidx(x, y);
                map.tiles[idx] = bsp.tiles[idx];
            }
        }
        for y in 0..HEIGHT as i32 {
            for x in center_x..WIDTH as i32 {
                let idx = mapidx(x, y);
                map.tiles[idx] = ca.tiles[idx];
            }
        }
        frames.push((map.clone(), "Why Not Both?".to_string()));

        let string_vec: Vec<char> = PREFAB
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();

        let mut i = 0;
        for y in 0..50 {
            for x in 0..11 {
                let p = Point::new(center_x - 4 + x, y);
                match string_vec[i] {
                    '.' => map.set(p, to_cp437('#'), RGB::named(YELLOW)),
                    '#' => map.set(p, to_cp437('.'), RGB::named(DARK_GRAY)),
                    _ => {}
                }
                i += 1;
            }
        }
        frames.push((map.clone(), "Just Add Prefab".to_string()));

        frames
    }
}

impl RoomBuilder {
    fn add_subrects(&mut self, rect: Rect) {
        let width = i32::abs(rect.x1 - rect.x2);
        let height = i32::abs(rect.y1 - rect.y2);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);

        self.rects
            .push(Rect::with_size(rect.x1, rect.y1, half_width, half_height));
        self.rects.push(Rect::with_size(
            rect.x1,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::with_size(
            rect.x1 + half_width,
            rect.y1,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::with_size(
            rect.x1 + half_width,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
    }

    fn get_random_rect(&mut self, rng: &mut RandomNumberGenerator) -> Rect {
        if self.rects.len() == 1 {
            return self.rects[0];
        }
        let idx = (rng.roll_dice(1, self.rects.len() as i32) - 1) as usize;
        self.rects[idx]
    }

    fn get_random_sub_rect(&self, rect: Rect, rng: &mut RandomNumberGenerator) -> Rect {
        let mut result = rect;
        let rect_width = i32::abs(rect.x1 - rect.x2);
        let rect_height = i32::abs(rect.y1 - rect.y2);

        let w = i32::max(3, rng.roll_dice(1, i32::min(rect_width, 10)) - 1) + 1;
        let h = i32::max(3, rng.roll_dice(1, i32::min(rect_height, 10)) - 1) + 1;

        result.x1 += rng.roll_dice(1, 6) - 1;
        result.y1 += rng.roll_dice(1, 6) - 1;
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    fn is_possible(&self, rect: Rect, map: &Map) -> bool {
        let mut expanded = rect;
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y2 += 2;

        let mut can_build = true;

        for y in expanded.y1..=expanded.y2 {
            for x in expanded.x1..=expanded.x2 {
                if x > WIDTH as i32 - 2 {
                    can_build = false;
                }
                if y > HEIGHT as i32 - 2 {
                    can_build = false;
                }
                if x < 1 {
                    can_build = false;
                }
                if y < 1 {
                    can_build = false;
                }
                if can_build {
                    if let Some(idx) = map.try_idx(Point::new(x, y)) {
                        if map.tiles[idx].0 != to_cp437('.') {
                            can_build = false;
                        }
                    } else {
                        can_build = false;
                    }
                }
            }
        }

        can_build
    }
}

fn apply_horizontal_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    use std::cmp::{max, min};
    for x in min(x1, x2)..=max(x1, x2) {
        if let Some(idx) = map.try_idx(Point::new(x, y)) {
            map.tiles[idx as usize] = (to_cp437('#'), RGB::named(YELLOW));
        }
    }
}

fn apply_vertical_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    use std::cmp::{max, min};
    for y in min(y1, y2)..=max(y1, y2) {
        if let Some(idx) = map.try_idx(Point::new(x, y)) {
            map.tiles[idx as usize] = (to_cp437('#'), RGB::named(YELLOW));
        }
    }
}

fn count_neighbors(map: &Map, x: usize, y: usize) -> usize {
    let mut n = 0;
    for ty in -1..=1 {
        for tx in -1..=1 {
            if !(ty == 0 && tx == 0) {
                if let Some(idx) = map.try_idx(Point::new(x as i32 + tx, y as i32 + ty)) {
                    if map.tiles[idx].0 == to_cp437('.') {
                        n += 1;
                    }
                }
            }
        }
    }
    n
}

fn iterate(map: &mut Map) {
    let map_copy = map.clone();
    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let neighbors = count_neighbors(&map_copy, x, y);
            if neighbors == 0 {
                map.set(Point::new(x, y), to_cp437('.'), RGB::named(DARK_GRAY));
            } else if neighbors < 5 {
                map.set(Point::new(x, y), to_cp437('#'), RGB::named(YELLOW));
            } else {
                map.set(Point::new(x, y), to_cp437('.'), RGB::named(DARK_GRAY));
            }
        }
    }
}

const PREFAB: &str = "
.#####.....
.....#.....
.#...#.....
.#.###.....
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#######.
.#.#.....#.
.#.#.....#.
.#.#.....#.
.#.......#.
.#.#######.
...........
...........
.#.#######.
.#.......#.
.#.#.....#.
.#.#.....#.
.#.#.....#.
.#.#######.
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.#.......
.#.###.....
.#...#.....
.#...#.....
.####......
";

fn main() -> BError {
    run(RoomBuilder::new())
}
