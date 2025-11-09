# Smiley Generator - Grid-Based Sketch Plan

## Overview
Create a grid-based generative art sketch that draws randomized smiley faces in each cell.

## Implementation Details

### 1. File Structure
- **File**: `notan_sketches/examples/smiley_gen.rs`
- **Pattern**: Follow `bobas-nightmare.rs` structure with Grid utilities

### 2. Grid Setup
- Randomized grid size: 1-20 rows × 1-20 cols
- Use `Grid::builder()` with cell data generation
- State management similar to bobas-nightmare

### 3. Smiley Components

#### Face
- Consistent circle across all grid cells
- Center: normalized (0.5, 0.5)
- Radius: ~0.4 normalized (80% of cell)

#### Eyes
- Two ellipses
- Randomized size (within reasonable bounds)
- Randomized placement in upper half of face
- Y position range: 0.2 - 0.45 (normalized)
- X positions: offset from center (e.g., 0.3 and 0.7)

#### Mouth
- Single ellipse
- Randomized size
- Centered horizontally (x: ~0.5)
- Y position range: 0.55 - 0.75 (normalized)

### 4. Data Structure

```rust
struct Eye {
    center: Vec2,     // normalized position
    radius: Vec2,     // normalized radii (x, y)
}

struct Mouth {
    center: Vec2,     // normalized position
    radius: Vec2,     // normalized radii (x, y)
}

struct SmileyData {
    face_center: Vec2,    // normalized (0.5, 0.5)
    face_radius: f32,     // ~0.4
    left_eye: Eye,
    right_eye: Eye,
    mouth: Mouth,
}
```

### 5. Features

#### Color Palette
- Use `PalettesSelection` for color variety
- Different colors for face, eyes, mouth
- Background color from palette

#### Keyboard Controls
- **R**: Regenerate with new seed and palette
- **G**: Toggle grid overlay
- **C**: Capture current frame

#### Rendering
- Store smiley data in normalized coordinates (0.0-1.0)
- Convert to pixels during draw using `cell.to_px()`
- All shapes drawn as filled ellipses/circles

#### Capture System
- 2x supersampling for antialiasing
- Save to `renders/smiley_gen/{seed}`
- Use `CapturingTexture` utility

### 6. Generation Function

```rust
fn generate_smiley_data(
    _row: u32,
    _col: u32,
    _bounds: Rect,
    rng: &mut Random,
) -> SmileyData {
    // Face: consistent
    let face_center = vec2(0.5, 0.5);
    let face_radius = 0.4;

    // Eyes: randomized in upper half
    let eye_y = rng.gen_range(0.2..0.45);
    let eye_radius_x = rng.gen_range(0.04..0.08);
    let eye_radius_y = rng.gen_range(0.06..0.12);

    let left_eye = Eye {
        center: vec2(0.35, eye_y),
        radius: vec2(eye_radius_x, eye_radius_y),
    };

    let right_eye = Eye {
        center: vec2(0.65, eye_y),
        radius: vec2(eye_radius_x, eye_radius_y),
    };

    // Mouth: randomized, centered horizontally
    let mouth_y = rng.gen_range(0.55..0.75);
    let mouth_radius_x = rng.gen_range(0.08..0.15);
    let mouth_radius_y = rng.gen_range(0.05..0.12);

    let mouth = Mouth {
        center: vec2(0.5, mouth_y),
        radius: vec2(mouth_radius_x, mouth_radius_y),
    };

    SmileyData {
        face_center,
        face_radius,
        left_eye,
        right_eye,
        mouth,
    }
}
```

### 7. Draw Implementation

For each cell:
1. Draw face circle (background/face color)
2. Draw left eye ellipse
3. Draw right eye ellipse
4. Draw mouth ellipse

All positions converted from normalized to pixels using `cell.to_px()`.

## Notes

- Follow the same random seed logging pattern as bobas-nightmare
- Use `get_draw_setup()` and `needs_redraw` flag for efficient rendering
- Grid overlay shows cell boundaries when enabled
- All coordinates stored normalized for resolution independence
