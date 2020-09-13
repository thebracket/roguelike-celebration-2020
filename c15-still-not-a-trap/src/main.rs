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

        let mut map = Map::new();
        let mut rng = RandomNumberGenerator::new();
        map.tiles.iter_mut().for_each(|(t, c)| {
            let roll = rng.range(0, 100);
            if roll < 55 {
                *t = to_cp437('.');
                *c = RGB::named(DARK_GRAY);
            } else {
                *t = to_cp437('#');
                *c = RGB::named(GREEN);
            }
        });

        for i in 0..10 {
            iterate(&mut map);
        }

        frames.push((map.clone(), "Cellular Automata Map".to_string()));

        let string_vec: Vec<char> = NOT_TRAP
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();

        loop {
            let base = Point::new(
                rng.range(1, WIDTH-10),
                rng.range(1, HEIGHT-10)
            );
            let mut can_build = true;
            let target = Rect::with_size(base.x, base.y, 6, 5);
            target.for_each(|p| {
                let idx = mapidx(p.x, p.y);
                if map.tiles[idx].0 != to_cp437('#') {
                    can_build = false;
                }
            });
            if can_build {
                let mut i = 0;
                for y in 0..5 {
                    for x in 0..6 {
                        let pt = Point::new(x, y) + base;
                        match string_vec[i] {
                            '$' => map.set(pt, to_cp437('$'), RGB::named(GOLD)),
                            '^' => map.set(pt, to_cp437('^'), RGB::named(RED)),
                            _ => {}
                        }
                        i += 1;
                    }
                }

                break;
            }
        }

        frames.push((map.clone(), "Found a place for the prefab".to_string()));

        frames
    }
}

const NOT_TRAP : &str = "
......
.^^^^.
.^$$^.
.^^^^.
......
";

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
                map.set(Point::new(x, y), to_cp437('#'), RGB::named(GREEN));
            } else {
                map.set(Point::new(x, y), to_cp437('.'), RGB::named(DARK_GRAY));
            }
        }
    }
}

fn main() -> BError {
    run(RoomBuilder::new())
}
