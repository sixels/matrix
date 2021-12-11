use rand::Rng;

/// Half width Katakana unicode characters
pub const GLYPH_CHAR_START: u32 = 0xFF61;
pub const GLYPH_CHAR_END: u32 = GLYPH_CHAR_START + 0x3C + 1;

/// A glyph stores a char with its position on screen
#[derive(Copy, Clone)]
pub struct Glyph {
    pub x: f32,
    pub y: f32,
    pub vel: f32,

    pub c: char,
}

impl Glyph {
    pub fn new(x: f32, y: f32, vel: f32, c: char) -> Self {
        Self { vel, x, y, c }
    }
}

/// Generate a number `size` of glyphs randomly
pub fn gen_glyphs(rng: &mut impl Rng, screen_width: u16, size: usize) -> Vec<Vec<Glyph>> {
    std::iter::repeat_with(|| gen_glyph(rng, screen_width))
        .take(size)
        .collect()
}

/// Generate a random glyph
pub fn gen_glyph(rng: &mut impl Rng, screen_width: u16) -> Vec<Glyph> {

    let (x, y) = random_xy(rng, screen_width as _);
    let velocity = random_velocity(rng);

    let root = Glyph::new(x, y, velocity, random_glyph_char(rng));

    let tail_len = rng.gen_range(3..13);
    let mut glyphs = Vec::with_capacity(tail_len + 1);

    glyphs.push(root);

    for i in 1..tail_len {
        let tail = Glyph::new(x, y - i as f32, velocity, random_glyph_char(rng));
        glyphs.push(tail)
    }

    glyphs
}

fn random_velocity(rng: &mut impl Rng) -> f32 {
    rng.gen_range(5.0..20.0)
}

fn random_xy(rng: &mut impl Rng, max_x: f32) -> (f32, f32) {
    (rng.gen_range(0.0..max_x), rng.gen_range(-80.0..-10.0))
}

fn random_glyph_char(rng: &mut impl Rng) -> char {
    char::from_u32(rng.gen_range(GLYPH_CHAR_START..GLYPH_CHAR_END)).unwrap()
}
