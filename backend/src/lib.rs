pub use bracket_lib::prelude::*;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;

#[derive(Clone)]
pub struct Map {
    pub tiles : Vec<(FontCharType, RGB)>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![(to_cp437('.'), RGB::named(DARK_GRAY)); WIDTH * HEIGHT]
        }
    }

    pub fn clear(&mut self, tile: FontCharType, color: RGB) {
        self.tiles.iter_mut().for_each(|t| {
            t.0 = tile;
            t.1 = color;
        })
    }

    pub fn clear_default(&mut self) {
        self.clear(to_cp437('.'), RGB::named(DARK_GRAY));
    }

    pub fn set(&mut self, position: Point, glyph: FontCharType, color: RGB) {
        let idx = (position.y as usize * WIDTH) + position.x as usize;
        self.tiles[idx] = (glyph, color);
    }

    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < WIDTH as i32 && point.y >= 0 && point.y < HEIGHT as i32
    }

    pub fn try_idx(&self, point : Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(mapidx(point.x, point.y))
        }
    }
}

pub fn mapidx(x: i32, y: i32) -> usize {
    ((y * WIDTH as i32) + x) as usize
}

pub trait MapGen{
    fn setup(&mut self);
    fn build(&mut self) -> Vec<(Map, String)>;
}

struct State {
    builder: Box<dyn MapGen>,
    frames: Vec<(Map, String)>,
    current_frame: usize
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let map = &self.frames[self.current_frame].0;
        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                let idx = (y * WIDTH) + x;
                ctx.set(
                    x,
                    y,
                    map.tiles[idx].1,
                    RGB::from_u8(0, 0, 0),
                    map.tiles[idx].0
                )
            }
        }

        ctx.print_centered(0, &self.frames[self.current_frame].1);

        let mut should_continue = true;
        if let Some(key) = ctx.key {
            if key == VirtualKeyCode::Return {
                self.current_frame += 1;
                if self.current_frame >= self.frames.len() {
                    should_continue = false;
                }
            }
        }

        if !should_continue { ctx.quit(); }
    }
}

pub fn run(gen: Box<dyn MapGen>) -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Map Builder")
        .build()?;

    let mut gs: State = State {
        builder: gen,
        frames: Vec::new(),
        current_frame: 0
    };

    gs.builder.setup();
    gs.frames = gs.builder.build();

    main_loop(context, gs)
}

pub fn iteration_color(iter: usize) -> RGB {
    match iter {
        0 => RGB::named(WHITE),
        1 => RGB::named(GREEN),
        2 => RGB::named(YELLOW),
        3 => RGB::named(BLUE),
        4 => RGB::named(MAGENTA),
        5 => RGB::named(CYAN),
        6 => RGB::named(WHITE),
        7 => RGB::named(DARK_GREEN),
        8 => RGB::named(BROWN1),
        9 => RGB::named(DARK_BLUE),
        10 => RGB::named(DARK_MAGENTA),
        11 => RGB::named(DARK_CYAN),
        12 => RGB::named(GRAY),
        13 => RGB::named(DARK_RED),
        _ => RGB::named(RED)
    }
}