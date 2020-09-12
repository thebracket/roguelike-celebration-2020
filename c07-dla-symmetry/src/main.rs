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
        let mut rng = RandomNumberGenerator::new();
        let mut map = Map::new();

        let starting_point = Point::new(WIDTH / 2, HEIGHT / 2);
        map.set(starting_point, to_cp437('#'), RGB::named(RED));
        map.set(
            starting_point + Point::new(1, 0),
            to_cp437('#'),
            RGB::named(RED),
        );
        map.set(
            starting_point + Point::new(-1, 0),
            to_cp437('#'),
            RGB::named(RED),
        );
        map.set(
            starting_point + Point::new(0, 1),
            to_cp437('#'),
            RGB::named(RED),
        );
        map.set(
            starting_point + Point::new(0, -1),
            to_cp437('#'),
            RGB::named(RED),
        );
        frames.push((map.clone(), "Starting Seed".to_string()));

        while map.tiles.iter().filter(|t| t.0 == to_cp437('#')).count() < (WIDTH * HEIGHT) / 3 {
            map.tiles
                .iter_mut()
                .filter(|t| t.1 == RGB::named(RED))
                .for_each(|t| t.1 = RGB::named(GREEN));

            let mut digger = Point::new(
                rng.roll_dice(1, WIDTH as i32 - 3) + 1,
                rng.roll_dice(1, HEIGHT as i32 - 3) + 1,
            );
            let mut digger_idx = mapidx(digger.x, digger.y);
            let mut prev = digger.clone();

            let mut path = line2d(
                LineAlg::Bresenham,
                digger,
                Point::new(WIDTH / 2, HEIGHT / 2),
            );

            while map.tiles[digger_idx].0 == to_cp437('.') && !path.is_empty() {
                prev = digger.clone();
                digger = path[0];
                digger_idx = mapidx(digger.x, digger.y);
                path.remove(0);
            }

            let center_x = WIDTH as i32 / 2;
            if center_x == prev.x {
                map.set(prev, to_cp437('#'), RGB::named(RED));
            } else {
                let dist_x = i32::abs(center_x - prev.x);
                let p1 = Point::new(center_x - dist_x, prev.y);
                let p2 = Point::new(center_x + dist_x, prev.y);
                map.set(p1, to_cp437('#'), RGB::named(RED));
                map.set(p2, to_cp437('#'), RGB::named(RED));
            }
            frames.push((map.clone(), "Iteration".to_string()));
        }

        frames
    }
}

fn main() -> BError {
    run(RoomBuilder::new())
}
