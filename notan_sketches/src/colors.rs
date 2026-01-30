use notan::app::Color;
use notan::random::rand::{
    distr::{Distribution, StandardUniform},
    prelude::IndexedRandom,
    rng, Rng,
};


// Blues
pub const PEACOCK: Color = Color::new(0.01, 0.18, 0.21, 1.0);
pub const AEGEAN: Color = Color::new(0.12, 0.27, 0.43, 1.0);
pub const AZURE: Color = Color::new(0.08, 0.13, 0.65, 1.0);
pub const CERULEAN: Color = Color::new(0.02, 0.57, 0.76, 1.0);
pub const STONE: Color = Color::new(0.35, 0.47, 0.56, 1.0);

// Reds Dark-Light
pub const GRAYPURP: Color = Color::new(0.29, 0.26, 0.36, 1.0);
pub const MAHOGANY: Color = Color::new(0.26, 0.05, 0.04, 1.0);
pub const CARMINE: Color = Color::new(0.59, 0.0, 0.09, 1.0);
pub const SCARLET: Color = Color::new(1.0, 0.14, 0.0, 1.0);
pub const SALMON: Color = Color::new(0.98, 0.5, 0.45, 1.0);
// Yellows
pub const OCHRE: Color = Color::new(0.8, 0.47, 0.13, 1.0);
pub const OLIVE: Color = Color::new(0.5, 0.5, 0.0, 1.0);
pub const SAFFRON: Color = Color::new(0.98, 0.54, 0.09, 1.0);
pub const BANANA: Color = Color::new(1.0, 0.88, 0.21, 1.0);
pub const LAGUNA: Color = Color::new(0.97, 0.89, 0.45, 1.0);

// Greens
pub const SACRAMENTO: Color = Color::new(0.02, 0.22, 0.15, 1.0);
pub const SEAWEED: Color = Color::new(0.21, 0.29, 0.13, 1.0);
pub const PICKLE: Color = Color::new(0.35, 0.49, 0.21, 1.0);
pub const LIME: Color = Color::new(0.78, 0.92, 0.27, 1.0);
pub const EMERALD: Color = Color::new(0.31, 0.79, 0.47, 1.0);


#[derive(Debug)]
pub enum PalettesSelection {
    All,
    RandomlyGenerated,
    Neon,
    PurpleFade,
    DarkAcademia,
    PartySoho,
    StabiloBossPastel,
}

//
// Implement random selection from Enum based on:
// https://stackoverflow.com/a/48491021
//
// Usage example with Notan rng:
// let palette: PalettesSelection = rng.gen();
//
impl Distribution<PalettesSelection> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PalettesSelection {
        // match rng.random_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.random_range(0..=6) {
            // rand 0.8
            0 => PalettesSelection::Neon,
            1 => PalettesSelection::PurpleFade,
            2 => PalettesSelection::DarkAcademia,
            3 => PalettesSelection::PartySoho,
            4 => PalettesSelection::StabiloBossPastel,
            5 => PalettesSelection::RandomlyGenerated,
            _ => PalettesSelection::All,
        }
    }
}

pub struct Palettes {
    pub all: Vec<Color>,
    pub random: Vec<Color>,
    pub neon: Vec<Color>,
    pub purple_fade: Vec<Color>,
    pub dark_academia: Vec<Color>,
    pub party_soho: Vec<Color>,
    pub stabilo_boss_pastel: Vec<Color>,
}

impl Palettes {
    fn generate_random() -> Vec<Color> {
        let mut rng = rng();
        vec![
            Color::new(
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                1.0,
            ),
            Color::new(
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                1.0,
            ),
            Color::new(
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                1.0,
            ),
            Color::new(
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                1.0,
            ),
            Color::new(
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                1.0,
            ),
            Color::new(
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                1.0,
            ),
        ]
    }
}


impl Default for Palettes {
    fn default() -> Self {
        Self {
            all: vec![
                PEACOCK, AEGEAN, AZURE, CERULEAN, STONE, OCHRE, OLIVE, SAFFRON, BANANA, LAGUNA,
                SACRAMENTO, SEAWEED, PICKLE, LIME, EMERALD, PICKLE, GRAYPURP, MAHOGANY, CARMINE,
                SCARLET, SALMON,
            ],
            random: Palettes::generate_random(),
            neon: vec![
                Color::new(1.0, 0.37, 0.0, 1.0),
                Color::new(0.8, 1.0, 0.0, 1.0),
                Color::new(0.74, 0.07, 1.0, 1.0),
            ],
            purple_fade: vec![
                Color::new(0.58, 0.0, 1.0, 1.0),
                Color::new(0.69, 0.26, 1.0, 1.0),
                Color::new(0.76, 0.46, 1.0, 1.0),
                Color::new(0.79, 0.55, 0.99, 1.0),
                Color::new(0.88, 0.69, 0.99, 1.0),
                Color::new(0.94, 0.81, 1.0, 1.0),
                Color::WHITE,
            ],
            // https://colorswall.com/palette/184115
            dark_academia: vec![
                Color::new(0.6, 0.41, 0.25, 1.0),
                Color::new(0.73, 0.64, 0.43, 1.0),
                Color::new(0.39, 0.2, 0.08, 1.0),
                Color::new(0.64, 0.5, 0.36, 1.0),
                Color::new(0.4, 0.34, 0.19, 1.0),
                Color::new(0.34, 0.24, 0.16, 1.0),
            ],
            // https://colorswall.com/palette/266406
            party_soho: vec![
                Color::BLACK,
                Color::new(0.0, 0.08, 0.16, 1.0),
                Color::new(0.0, 0.0, 0.08, 1.0),
                Color::new(0.0, 0.08, 0.08, 1.0),
                Color::new(0.0, 0.08, 0.24, 1.0),
                Color::new(0.08, 0.16, 0.31, 1.0),
            ],
            stabilo_boss_pastel: vec![
                Color::new(0.97, 0.87, 0.51, 1.0),
                Color::new(0.96, 0.67, 0.56, 1.0),
                Color::new(0.96, 0.71, 0.75, 1.0),
                Color::new(0.84, 0.71, 0.84, 1.0),
                Color::new(0.73, 0.87, 0.85, 1.0),
                Color::new(0.61, 0.82, 0.72, 1.0),
            ],
        }
    }
}

impl Palettes {
    pub fn choose_color(palette_selection: &PalettesSelection) -> Color {
        let palettes = Palettes::default();
        let palette = match palette_selection {
            PalettesSelection::All => palettes.all,
            PalettesSelection::RandomlyGenerated => palettes.random,
            PalettesSelection::Neon => palettes.neon,
            PalettesSelection::PurpleFade => palettes.purple_fade,
            PalettesSelection::DarkAcademia => palettes.dark_academia,
            PalettesSelection::PartySoho => palettes.party_soho,
            PalettesSelection::StabiloBossPastel => palettes.stabilo_boss_pastel,
        };
        let mut rng = rng();
        if let Some(color) = palette.choose(&mut rng) {
            return *color;
        }
        Color::GRAY
    }
}
