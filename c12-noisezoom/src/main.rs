use backend::*;

struct RoomBuilder {}

impl RoomBuilder {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

fn noise_map(
    seed: u64,
    octaves: i32,
    gain: f32,
    lacunarity: f32,
    freq: f32,
    frames: &mut Vec<(Map, String)>,
    x_scale: f32,
    y_scale: f32,
    title: &str,
) {
    let mut map = Map::new();
    let mut noise = FastNoise::seeded(seed);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(octaves);
    noise.set_fractal_gain(gain);
    noise.set_fractal_lacunarity(lacunarity);
    noise.set_frequency(freq);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let n = noise.get_noise(x as f32 * (x_scale * 0.5), y as f32 * (y_scale * 0.5));
            if n < 0.0 {
                map.set(
                    Point::new(x, y),
                    to_cp437('~'),
                    RGB::from_f32(0.0, 0.0, n + 0.75),
                );
            } else if n < 0.5 {
                map.set(
                    Point::new(x, y),
                    to_cp437(';'),
                    RGB::from_f32(0.0, n + 0.25, 0.0),
                );
            } else {
                map.set(Point::new(x, y), to_cp437('^'), RGB::from_f32(n, n, n));
            }
        }
    }
    frames.push((map.clone(), title.to_string()));
}

impl MapGen for RoomBuilder {
    fn setup(&mut self) {}

    fn build(&mut self) -> Vec<(Map, String)> {
        let mut frames = Vec::new();

        let seed = 4;
        let octaves = 3;
        let gain = 0.005;
        let lacunarity = 4.0;
        let freq = 0.08;

        noise_map(
            seed,
            octaves,
            gain,
            lacunarity,
            freq,
            &mut frames,
            1.0,
            1.0,
            "Zoomed Out",
        );
        let mut scale = 1.0;
        while scale > 0.1 {
            noise_map(
                seed,
                octaves,
                gain,
                lacunarity,
                freq,
                &mut frames,
                scale,
                scale,
                &format!("Scale {}", scale),
            );
            scale -= 0.01;
        }

        frames
    }
}

fn main() -> BError {
    run(RoomBuilder::new())
}
