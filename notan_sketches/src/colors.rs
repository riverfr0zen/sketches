use notan::app::Color;
use notan::random::rand::prelude::SliceRandom;
use notan::random::rand::thread_rng;

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
    Neon,
}

pub struct Palettes {
    pub all: Vec<Color>,
    pub neon: Vec<Color>,
}

impl Default for Palettes {
    fn default() -> Self {
        Self {
            all: vec![
                PEACOCK, AEGEAN, AZURE, CERULEAN, STONE, OCHRE, OLIVE, SAFFRON, BANANA, LAGUNA,
                SACRAMENTO, SEAWEED, PICKLE, LIME, EMERALD, PICKLE, GRAYPURP, MAHOGANY, CARMINE,
                SCARLET, SALMON,
            ],
            neon: vec![
                Color::new(1.0, 0.37, 0.0, 1.0),
                Color::new(0.8, 1.0, 0.0, 1.0),
                Color::new(0.74, 0.07, 1.0, 1.0),
            ],
        }
    }
}

impl Palettes {
    pub fn choose_color(palette_selection: &PalettesSelection) -> Color {
        let palettes = Palettes::default();
        let palette = match palette_selection {
            PalettesSelection::All => palettes.all,
            PalettesSelection::Neon => palettes.neon,
        };
        let mut rng = thread_rng();
        if let Some(color) = palette.choose(&mut rng) {
            return *color;
        }
        Color::GRAY
    }
}
