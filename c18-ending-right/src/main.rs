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

        for _ in 0..10 {
            iterate(&mut map);
        }

        // Find a central starting point
        let start = map
            .tiles
            .iter()
            .enumerate()
            .map(|(i, (tt, _col))| (i, *tt))
            .filter(|(_i, tt)| *tt == to_cp437('#'))
            .map(|(i, _tt)| {
                (
                    i,
                    DistanceAlg::Pythagoras.distance2d(
                        Point::new(WIDTH / 2, HEIGHT / 2),
                        Point::new(i % WIDTH, i / WIDTH),
                    ),
                )
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        // Build a Dijkstra Map
        let dijkstra = DijkstraMap::new(WIDTH, HEIGHT, &[start], &map, 1024.0);
        for (i, d) in dijkstra.map.iter().enumerate() {
            if map.tiles[i].0 == to_cp437('#') {
                if *d > 2000.0 {
                    map.set(
                        Point::new(i % WIDTH, i / WIDTH),
                        to_cp437('.'),
                        RGB::named(DARK_GRAY),
                    )
                }
            }
        }

        // Preferred start
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
        map.set(
            Point::new(start % WIDTH, start / WIDTH),
            to_cp437('@'),
            RGB::named(GOLD),
        );

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

        let path = a_star_search(start, end, &map);
        for p in path.steps.iter() {
            map.set(
                Point::new(p % WIDTH, p / WIDTH),
                to_cp437('*'),
                RGB::named(PURPLE),
            );
        }
        map.set(
            Point::new(start % WIDTH, start / WIDTH),
            to_cp437('@'),
            RGB::named(GOLD),
        );
        map.set(desired_end, to_cp437('!'), RGB::named(RED));
        map.set(
            Point::new(end % WIDTH, end / WIDTH),
            to_cp437('>'),
            RGB::named(GOLD),
        );

        frames.push((map.clone(), " Exit by direction ".to_string()));

        frames
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
