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

        let starting_point = Point::new(WIDTH/2, HEIGHT/2);
        map.set(starting_point, to_cp437('#'), RGB::named(RED));
        map.set(starting_point + Point::new(1,0), to_cp437('#'), RGB::named(RED));
        map.set(starting_point + Point::new(-1,0), to_cp437('#'), RGB::named(RED));
        map.set(starting_point + Point::new(0,1), to_cp437('#'), RGB::named(RED));
        map.set(starting_point + Point::new(0,-1), to_cp437('#'), RGB::named(RED));
        frames.push((map.clone(), "Starting Seed".to_string()));

        while map.tiles.iter().filter(|t| t.0 == to_cp437('#')).count() < (WIDTH * HEIGHT) / 3 {
            map.tiles
                .iter_mut()
                .filter(|t| t.1 == RGB::named(RED))
                .for_each(|t| t.1 = RGB::named(GREEN));

            let mut digger = Point::new(
                rng.roll_dice(1, WIDTH as i32 - 3)+1,
                rng.roll_dice(1, HEIGHT as i32 - 3)+1,
            );
            let mut prev = digger.clone();
            let mut digger_idx = mapidx(digger.x, digger.y);
            while map.tiles[digger_idx].0 == to_cp437('.') {
                prev = digger.clone();
                let stagger_direction = rng.roll_dice(1, 4);
                match stagger_direction {
                    1 => { if digger.x > 2 { digger.x -= 1; } }
                    2 => { if digger.x < WIDTH as i32 -2 { digger.x += 1; } }
                    3 => { if digger.y > 2 { digger.y -=1; } }
                    _ => { if digger.y < HEIGHT as i32 -2 { digger.y += 1; } }
                }
                digger_idx = mapidx(digger.x, digger.y);
            }
            map.set(prev, to_cp437('#'), RGB::named(RED));
            frames.push((map.clone(), "Iteration".to_string()));
        }

        frames
    }
}

fn main() -> BError {
    run(RoomBuilder::new())
}
