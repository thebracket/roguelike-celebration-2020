use backend::*;

struct RoomBuilder {
}

impl RoomBuilder {
    fn new() -> Box<Self> {
        Box::new(Self{})
    }
}

impl MapGen for RoomBuilder {
    fn setup(&mut self) {}

    fn build(&mut self) -> Vec<(Map, String)> {
        let mut frames = Vec::new();
        let mut rng = RandomNumberGenerator::new();
        let mut map = Map::new();

        frames.push((map.clone(), "Start Solid".to_string()));

        drunk(&mut map, Point::new(WIDTH/2, HEIGHT/2), &mut rng);
        frames.push((map.clone(), "First Drunken Digger".to_string()));

        let mut i = 2;
        while map.tiles.iter().filter(|t| t.0 == to_cp437('#')).count() < (WIDTH*HEIGHT)/3 {
            map.tiles.iter_mut().filter(|t| t.1 == RGB::named(RED)).for_each(|t| t.1 = RGB::named(GREEN));

            let open_tiles : Vec<usize> = map
                .tiles
                .iter()
                .enumerate()
                .filter(|(_,t)| t.0 == to_cp437('#'))
                .map(|(i,_)| i)
                .collect();
            let target = rng.random_slice_entry(&open_tiles);
            if let Some(target) = target {
                drunk(&mut map, Point::new(target % WIDTH, target / WIDTH), &mut rng);
                frames.push((map.clone(), format!("Drunken Digger {}", i)));
                i += 1;
            }
        }

        frames
    }
}

fn drunk(map: &mut Map, start: Point, rng: &mut RandomNumberGenerator) {
    let mut steps = 0;
    let mut pos = start;
    loop {
        let delta = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };
        pos = pos + delta;
        if let Some(_idx) = map.try_idx(pos) {
            map.set(pos, to_cp437('#'), RGB::named(RED));
        } else {
            break;
        }

        steps += 1;
        if steps > 200 {
            break;
        }
    }
}

fn main() -> BError {
    run(RoomBuilder::new())
}
