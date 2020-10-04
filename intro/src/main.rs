use backend::*;

#[derive(PartialEq, Clone)]
enum Direction {
    FadeIn,
    FadeOut,
}

#[derive(Clone)]
struct Entry {
    char: FontCharType,
    intensity: f32,
    direction: Direction,
}

impl Entry {
    fn new(rng: &mut RandomNumberGenerator) -> Self {
        Self {
            char: rng.range(0, 255),
            intensity: rng.range(0, 255) as f32 / 255.0,
            direction: match rng.roll_dice(1, 2) {
                1 => Direction::FadeIn,
                _ => Direction::FadeOut,
            },
        }
    }

    fn draw(&self, x: usize, y: usize, ctx: &mut BTerm) {
        let i = self.intensity / 2.0;
        ctx.set(x, y, RGB::from_f32(i, i, i), RGB::named(BLACK), self.char);
    }

    fn update(&mut self, rng: &mut RandomNumberGenerator) {
        const STEP: f32 = 0.05;
        if self.direction == Direction::FadeIn {
            self.intensity += STEP;
            if self.intensity > 1.0 {
                self.direction = Direction::FadeOut;
            }
        } else {
            self.intensity -= STEP;
            if self.intensity < 0.0 {
                self.direction = Direction::FadeIn;
                self.char = rng.range(0, 255);
            }
        }
    }
}

#[derive(Clone)]
struct Line {
    entries: Vec<Entry>,
}

impl Line {
    fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut entries = Vec::new();
        for _ in 0..80 {
            entries.push(Entry::new(rng))
        }
        Line { entries }
    }

    fn draw(&self, y: usize, ctx: &mut BTerm) {
        for (x, c) in self.entries.iter().enumerate() {
            c.draw(x, y, ctx);
        }
    }

    fn update(&mut self, rng: &mut RandomNumberGenerator) {
        self.entries.iter_mut().for_each(|entry| entry.update(rng));
    }
}

struct Intro {
    rng: RandomNumberGenerator,
    lines: Vec<Line>,
}

impl GameState for Intro {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.lines
            .iter()
            .enumerate()
            .for_each(|(y, l)| l.draw(y, ctx));

        let mut rng = RandomNumberGenerator::new();
        self.lines.iter_mut().for_each(|line| line.update(&mut rng));

        self.lines.insert(0, self.lines[49].clone());
        self.lines.remove(50);

        // Print logo
        ctx.print_color_centered(
            40,
            RGB::named(GRAY),
            RGB::named(BLACK),
            "Bracket Productions",
        );
    }
}

impl Intro {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut lines = Vec::new();
        for _ in 0..50 {
            lines.push(Line::new(&mut rng));
        }
        Intro { lines, rng }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Intro")
        .with_fps_cap(15.0)
        .build()?;

    main_loop(context, Intro::new())
}
