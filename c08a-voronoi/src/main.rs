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

        // Seed it
        let mut seeds = Vec::new();
        for _ in 0..16 {
            seeds.push(Point::new(
                rng.range(1, WIDTH - 1),
                rng.range(1, HEIGHT - 1),
            ));
        }

        for (i, s) in seeds.iter().enumerate() {
            map.set(*s, to_cp437('*'), iteration_color(i))
        }
        frames.push((map.clone(), "Initial Seeds".to_string()));

        // Allocate tiles
        map = Map::new();
        let mut membership = vec![0; WIDTH * HEIGHT];
        for (i, m) in membership.iter_mut().enumerate() {
            let my_pos = Point::new(i % WIDTH, i / WIDTH);
            let closest = seeds
                .iter()
                .enumerate()
                .map(|(i, pos)| (i, DistanceAlg::Pythagoras.distance2d(my_pos, *pos)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0;
            *m = closest;
        }
        for (i, m) in membership.iter().enumerate() {
            let my_pos = Point::new(i % WIDTH, i / WIDTH);
            map.set(my_pos, to_cp437('#'), iteration_color(*m));
        }
        frames.push((map.clone(), "Closest Membership (Pythagoras)".to_string()));
        let membership_py = membership.clone();

        // Allocate Tiles - this time with a different heuristic
        map = Map::new();
        let mut membership = vec![0; WIDTH * HEIGHT];
        for (i, m) in membership.iter_mut().enumerate() {
            let my_pos = Point::new(i % WIDTH, i / WIDTH);
            let closest = seeds
                .iter()
                .enumerate()
                .map(|(i, pos)| (i, DistanceAlg::Manhattan.distance2d(my_pos, *pos)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0;
            *m = closest;
        }
        for (i, m) in membership.iter().enumerate() {
            let my_pos = Point::new(i % WIDTH, i / WIDTH);
            map.set(my_pos, to_cp437('#'), iteration_color(*m));
        }
        frames.push((map.clone(), "Closest Membership (Manhattan)".to_string()));

        // Allocate Tiles - this time with a different heuristic
        map = Map::new();
        let mut membership = vec![0; WIDTH * HEIGHT];
        for (i, m) in membership.iter_mut().enumerate() {
            let my_pos = Point::new(i % WIDTH, i / WIDTH);
            let closest = seeds
                .iter()
                .enumerate()
                .map(|(i, pos)| (i, DistanceAlg::Chebyshev.distance2d(my_pos, *pos)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0;
            *m = closest;
        }
        for (i, m) in membership.iter().enumerate() {
            let my_pos = Point::new(i % WIDTH, i / WIDTH);
            map.set(my_pos, to_cp437('#'), iteration_color(*m));
        }
        frames.push((map.clone(), "Closest Membership (Chebyshev)".to_string()));

        // Find edges
        map = Map::new();
        for i in 0..WIDTH * HEIGHT {
            let my_pos = Point::new(i % WIDTH, i / WIDTH);
            if my_pos.x == 0
                || my_pos.x == WIDTH as i32 - 1
                || my_pos.y == 0
                || my_pos.y == HEIGHT as i32 - 1
            {
                map.set(my_pos, to_cp437('.'), RGB::named(DARK_GRAY));
            } else {
                if membership_py[i] != membership_py[i + 1]
                    || membership_py[i] != membership_py[i + WIDTH]
                {
                    map.set(my_pos, to_cp437('#'), RGB::named(YELLOW));
                }
            }
        }
        frames.push((map.clone(), "Voronoi Boundary Walls".to_string()));

        frames
    }
}

fn main() -> BError {
    run(RoomBuilder::new())
}
