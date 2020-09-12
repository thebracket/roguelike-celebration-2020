use backend::*;

struct RoomBuilder {}

impl RoomBuilder {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

fn noise_map(octaves: i32, gain: f32, lacunarity: f32, freq: f32, frames: &mut Vec<(Map, String)>) {
    let mut map = Map::new();
    let mut noise = FastNoise::seeded(2);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(octaves);
    noise.set_fractal_gain(gain);
    noise.set_fractal_lacunarity(lacunarity);
    noise.set_frequency(freq);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let n = ((noise.get_noise(x as f32 / 50.0, y as f32 / 25.0)) + 1.0) / 2.0;
            map.set(Point::new(x, y), to_cp437('#'), RGB::from_f32(n, n, n));
        }
    }
    frames.push((
        map.clone(),
        format!(
            "Ocataves: {}, Gain: {}, Lacunarity: {}, Frequency: {}",
            octaves, gain, lacunarity, freq
        ),
    ));
}

impl MapGen for RoomBuilder {
    fn setup(&mut self) {}

    fn build(&mut self) -> Vec<(Map, String)> {
        let mut frames = Vec::new();

        noise_map(1, 0.2, 1.0, 1.0, &mut frames);
        noise_map(10, 0.2, 1.0, 1.0, &mut frames);
        noise_map(10, 0.5, 1.0, 1.0, &mut frames);
        noise_map(10, 0.2, 5.0, 1.0, &mut frames);
        noise_map(10, 0.2, 5.0, 5.0, &mut frames);

        frames
    }
}

fn main() -> BError {
    run(RoomBuilder::new())
}
