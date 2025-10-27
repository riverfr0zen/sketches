# Grid Utilities Module Plan

## Implementation Progress

- [ ] Step 1: Create `notan_sketches/src/gridutils.rs` with core types
- [ ] Step 2: Implement `Grid<T>` struct with storage and metadata
- [ ] Step 3: Implement `GridBuilder<T>` with builder pattern
- [ ] Step 4: Implement `CellContext<T>` and `CellContextMut<T>` with ALL coordinate helpers
  - [ ] Normalized coordinate methods (PRIMARY): `norm()`, `norm_abs()`, `to_norm()`, etc.
  - [ ] Pixel coordinate methods (SECONDARY): `abs()`, `to_local()`, `center()`, etc.
  - [ ] Canvas-wide helpers on Grid: `norm_to_pixels()`, `pixels_to_norm()`
- [ ] Step 5: Implement iteration methods (`cells()`, `cells_mut()`)
- [ ] Step 6: Implement random access (`get()`, `get_mut()`)
- [ ] Step 7: Implement bulk operations (`regenerate_cells()`)
- [ ] Step 8: Implement debug helpers (`draw_overlay()`)
- [ ] Step 9: Write comprehensive tests (see Testing Strategy section)
- [ ] Step 10: Add module to `lib.rs`
- [ ] Step 11: Write documentation emphasizing normalized coordinates as primary approach
- [ ] Step 12: (Optional) Refactor `claudes_first.rs` as demonstration

---

## Goal
Create a reusable grid utilities module (`gridutils.rs`) to eliminate boilerplate when creating grid-based generative art sketches in Notan.

## Problem Statement

Currently, creating grid-based sketches requires significant boilerplate:
- Manual calculation of tile dimensions
- Manual management of per-cell data in separate `Vec<T>` collections
- Manual index tracking when iterating through rows/columns
- Repetitive coordinate transformation calculations (cell-relative to absolute)
- Duplicated grid rendering code for debugging

Example from `claudes_first.rs` (lines 16-145, 387-418):
- ~130 lines of grid setup and iteration logic
- Multiple parallel arrays for per-cell data
- Manual tile_index management
- Repeated coordinate calculations

## Design Overview

### Core Components

1. **`Grid<T>` struct** - Lives in `AppState`, encapsulates grid structure and per-cell data
2. **`CellContext<'a, T>` struct** - Provided during iteration, contains cell metadata and data reference
3. **`GridBuilder<T>` struct** - Builder pattern for ergonomic Grid construction
4. **Iteration APIs** - Support for both `update()` and `draw()` use cases
5. **Normalized Coordinate System** - Resolution-independent 0.0-1.0 coordinate space (PRIMARY approach)

### Ownership Model

The `Grid<T>` is owned by `AppState` and persists across frames:

```rust
#[derive(AppState)]
struct State {
    rng: Random,
    grid: Grid<CellData>,  // Owns grid structure + all cell data
    // ... other state
}

struct CellData {
    position: Vec2,  // RECOMMENDED: Store in normalized 0.0-1.0 space
    color: Color,
    // ... arbitrary per-cell state
}
```

## Coordinate System Philosophy

**PRIMARY APPROACH: Normalized Coordinates (0.0 - 1.0)**

Authors should work primarily in normalized coordinate space (0.0 to 1.0) for resolution independence. This makes sketches portable across different work sizes and aligns with shader conventions.

**Four Coordinate Spaces:**
1. **Cell-local normalized** (0.0-1.0) - Position within a cell, resolution-independent
2. **Cell-local pixels** - Position within a cell in pixels
3. **Absolute normalized** (0.0-1.0) - Position on canvas, resolution-independent
4. **Absolute pixels** - Position on canvas in pixels

The API provides conversions between all spaces, with **normalized coordinates as the recommended default**.

## API Design

### 1. Grid Creation (Builder Pattern)

```rust
// Simple grid with no per-cell data
let grid = Grid::<()>::new(rows, cols, work_size);

// Grid with generated cell data - RECOMMENDED: use normalized coordinates (0.0-1.0)
let grid = Grid::builder(rows, cols, work_size)
    .with_cell_data(|row, col, bounds, rng| -> CellData {
        CellData {
            // Work in normalized 0.0-1.0 space (resolution-independent!)
            position: vec2(
                rng.gen_range(0.1..0.9),  // 10% margin
                rng.gen_range(0.1..0.9),
            ),
            color: some_palette.choose(rng),
        }
    })
    .build(&mut rng);

// Alternative: Use pixel coordinates if needed (not recommended)
let grid = Grid::builder(rows, cols, work_size)
    .with_cell_data(|row, col, bounds, rng| -> CellData {
        CellData {
            position: vec2(
                rng.gen_range(margin..(bounds.width - margin)),
                rng.gen_range(margin..(bounds.height - margin))
            ),
            color: some_palette.choose(rng),
        }
    })
    .build(&mut rng);
```

### 2. Iteration for Reading (in `draw()`)

```rust
// RECOMMENDED: Use normalized coordinates
for cell in state.grid.cells() {
    // Convert normalized cell position (0-1) to absolute pixels for drawing
    let abs_pos = cell.norm(cell.data.position);  // Short method name!

    draw.circle(radius)
        .position(abs_pos.x, abs_pos.y)
        .color(cell.data.color);
}

// Alternative: If you stored pixel coordinates
for cell in state.grid.cells() {
    let abs_pos = cell.abs(cell.data.position);  // Convert local pixels to absolute
    draw.circle(radius)
        .position(abs_pos.x, abs_pos.y)
        .color(cell.data.color);
}

// Working with canvas-wide normalized coordinates
for cell in state.grid.cells() {
    let canvas_norm = cell.norm_abs(cell.data.position);  // 0-1 on entire canvas
    // Use for cross-cell effects, shader uniforms, etc.
}

// Access cell metadata
println!("Drawing cell at ({}, {})", cell.row, cell.col);
println!("Cell bounds: {:?}", cell.bounds);  // Local cell rectangle
println!("Tile offset: {:?}", cell.offset);  // Absolute position of cell origin
```

### 3. Iteration for Mutation (in `update()`)

```rust
// RECOMMENDED: Mutable iteration with normalized coordinates
for mut cell in state.grid.cells_mut() {
    cell.data.position += velocity * dt;

    // Clamp to normalized bounds (0.0 to 1.0)
    cell.data.position.x = cell.data.position.x.clamp(0.0, 1.0);
    cell.data.position.y = cell.data.position.y.clamp(0.0, 1.0);
}

// Alternative: If using pixel coordinates
for mut cell in state.grid.cells_mut() {
    cell.data.position += velocity * dt;

    // Clamp to pixel bounds
    cell.data.position.x = cell.data.position.x.clamp(0.0, cell.bounds.width);
    cell.data.position.y = cell.data.position.y.clamp(0.0, cell.bounds.height);
}
```

### 4. Bulk Regeneration

```rust
// RECOMMENDED: Regenerate with normalized coordinates
state.grid.regenerate_cells(&mut state.rng, |row, col, bounds, rng| {
    CellData {
        position: vec2(
            rng.gen_range(0.0..1.0),  // Normalized!
            rng.gen_range(0.0..1.0),
        ),
        color: palette.choose(rng),
    }
});

// Alternative: With pixel coordinates
state.grid.regenerate_cells(&mut state.rng, |row, col, bounds, rng| {
    CellData {
        position: vec2(
            rng.gen_range(0.0..bounds.width),
            rng.gen_range(0.0..bounds.height)
        ),
        color: palette.choose(rng),
    }
});
```

### 5. Random Access

```rust
// Get specific cell (immutable)
if let Some(cell) = state.grid.get(row, col) {
    // Read cell.data
}

// Get specific cell (mutable)
if let Some(mut cell) = state.grid.get_mut(row, col) {
    // Modify cell.data
}
```

### 6. Grid Overlay Rendering (Debug Helper)

```rust
// Optional: render grid lines for debugging
state.grid.draw_overlay(&mut draw, Color::GREEN, 2.0);
```

## CellContext API

The `CellContext` struct provides cell metadata and coordinate helpers:

```rust
pub struct CellContext<'a, T> {
    pub row: u32,
    pub col: u32,
    pub bounds: Rect,      // Cell rectangle in local coordinates (0,0 to width,height)
    pub offset: Vec2,      // Absolute position of cell's top-left corner
    pub data: &'a T,       // Reference to cell-specific data
    // ... internal grid reference
}

impl<'a, T> CellContext<'a, T> {
    // ===== PRIMARY METHODS: Normalized Coordinates (RECOMMENDED) =====

    // Convert cell-local normalized (0-1) to absolute pixels
    // SHORT NAME for common use! Most frequently used method.
    pub fn norm(&self, norm_local: Vec2) -> Vec2;

    // Convert cell-local normalized (0-1) to canvas-wide normalized (0-1)
    pub fn norm_abs(&self, norm_local: Vec2) -> Vec2;

    // Reverse: absolute pixels to cell-local normalized (0-1)
    pub fn to_norm(&self, abs_pixels: Vec2) -> Vec2;

    // Reverse: canvas-wide normalized (0-1) to cell-local normalized (0-1)
    pub fn to_norm_local(&self, norm_abs: Vec2) -> Vec2;

    // Get center point in cell-local normalized (always vec2(0.5, 0.5))
    pub fn center_norm(&self) -> Vec2;

    // Get center point in canvas-wide normalized (0-1)
    pub fn center_norm_abs(&self) -> Vec2;

    // ===== SECONDARY METHODS: Pixel Coordinates =====

    // Convert cell-local pixels to absolute pixels
    pub fn abs(&self, local_pixels: Vec2) -> Vec2;

    // Convert absolute pixels to cell-local pixels
    pub fn to_local(&self, abs_pixels: Vec2) -> Vec2;

    // Get center point in absolute pixels
    pub fn center(&self) -> Vec2;

    // ===== METADATA =====

    // Get cell index (flattened row-major index)
    pub fn index(&self) -> usize;
}
```

### Coordinate Helper Summary

| Method | Input | Output | Use Case |
|--------|-------|--------|----------|
| `norm()` | Cell norm (0-1) | Abs pixels | **Most common**: Drawing with normalized data |
| `norm_abs()` | Cell norm (0-1) | Canvas norm (0-1) | Cross-cell effects, shaders |
| `to_norm()` | Abs pixels | Cell norm (0-1) | Convert mouse/touch to normalized |
| `to_norm_local()` | Canvas norm (0-1) | Cell norm (0-1) | Shader to cell coords |
| `abs()` | Cell pixels | Abs pixels | Drawing with pixel data |
| `to_local()` | Abs pixels | Cell pixels | Convert mouse/touch to pixels |

## Implementation Details

### Grid Structure

```rust
pub struct Grid<T> {
    rows: u32,
    cols: u32,
    work_size: Vec2,
    cell_width: f32,
    cell_height: f32,
    cells: Vec<T>,  // Flattened row-major storage
}
```

### Key Methods

```rust
impl<T> Grid<T> {
    pub fn new(rows: u32, cols: u32, work_size: Vec2) -> Grid<()>;
    pub fn builder(rows: u32, cols: u32, work_size: Vec2) -> GridBuilder<T>;

    pub fn cells(&self) -> impl Iterator<Item = CellContext<'_, T>>;
    pub fn cells_mut(&mut self) -> impl Iterator<Item = CellContextMut<'_, T>>;

    pub fn get(&self, row: u32, col: u32) -> Option<CellContext<'_, T>>;
    pub fn get_mut(&mut self, row: u32, col: u32) -> Option<CellContextMut<'_, T>>;

    pub fn regenerate_cells<F>(&mut self, rng: &mut Random, f: F)
        where F: Fn(u32, u32, Rect, &mut Random) -> T;

    pub fn draw_overlay(&self, draw: &mut Draw, color: Color, stroke_width: f32);

    // Metadata accessors
    pub fn rows(&self) -> u32;
    pub fn cols(&self) -> u32;
    pub fn cell_width(&self) -> f32;
    pub fn cell_height(&self) -> f32;
    pub fn total_cells(&self) -> usize;

    // Canvas-wide normalized coordinate helpers
    pub fn norm_to_pixels(&self, norm_pos: Vec2) -> Vec2;  // Canvas norm (0-1) to abs pixels
    pub fn pixels_to_norm(&self, abs_pos: Vec2) -> Vec2;   // Abs pixels to canvas norm (0-1)
}
```

## Advanced Features (Optional/Future)

1. **Neighbor Access**: `cell.neighbors()` iterator for cross-cell patterns
2. **Sub-grid Iteration**: `grid.region(row_range, col_range)` for partial iteration
3. **Coordinate Mapping**: Support for different grid layouts (hex, triangular)
4. **Shader Integration**: Helper to generate uniform data for shaders (like `TileGridInfo`)

## Testing Strategy

Testing is critical for this module due to the mathematical nature of coordinate transformations and the importance of getting indexing right.

### Test Categories

#### 1. Core Grid Functionality
```rust
#[test]
fn test_grid_creation_dimensions()
fn test_grid_cell_count()
fn test_cell_dimensions_calculated_correctly()
fn test_grid_with_different_aspect_ratios()
fn test_single_cell_grid()
fn test_large_grid()
```

#### 2. Cell Indexing
```rust
#[test]
fn test_row_col_to_index()
fn test_cell_iteration_visits_all_cells_once()
fn test_cell_iteration_order_is_row_major()
fn test_get_returns_correct_cell()
fn test_get_out_of_bounds_returns_none()
```

#### 3. Coordinate Transformations - Normalized (CRITICAL!)
```rust
#[test]
fn test_norm_converts_cell_normalized_to_abs_pixels()
fn test_norm_at_origin()  // vec2(0.0, 0.0) -> tile offset
fn test_norm_at_center()  // vec2(0.5, 0.5) -> tile center
fn test_norm_at_max()     // vec2(1.0, 1.0) -> tile bottom-right
fn test_norm_abs_converts_to_canvas_normalized()
fn test_to_norm_reverse_conversion()
fn test_center_norm_always_returns_half()
fn test_center_norm_abs_varies_by_cell()
```

#### 4. Coordinate Transformations - Pixels
```rust
#[test]
fn test_abs_converts_local_pixels_to_absolute()
fn test_to_local_reverse_conversion()
fn test_center_returns_absolute_pixel_center()
```

#### 5. Canvas-Wide Helpers
```rust
#[test]
fn test_grid_norm_to_pixels_full_canvas()
fn test_grid_pixels_to_norm_full_canvas()
fn test_roundtrip_canvas_conversions()
```

#### 6. Builder Pattern
```rust
#[test]
fn test_builder_with_cell_data()
fn test_builder_passes_correct_bounds_to_closure()
fn test_builder_with_empty_data()
```

#### 7. Iteration
```rust
#[test]
fn test_cells_iterator_is_immutable()
fn test_cells_mut_allows_mutation()
fn test_iteration_provides_correct_row_col()
fn test_iteration_provides_correct_bounds()
fn test_iteration_provides_correct_offset()
```

#### 8. Data Management
```rust
#[test]
fn test_regenerate_cells_replaces_all_data()
fn test_get_mut_allows_modification()
fn test_cell_data_persists_across_iterations()
```

#### 9. Edge Cases
```rust
#[test]
fn test_non_square_cells()
fn test_very_small_work_size()
fn test_very_large_work_size()
fn test_fractional_cell_dimensions()
```

### Test Organization

Tests should be in a `#[cfg(test)]` module at the end of `gridutils.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use notan::math::vec2;
    use notan::prelude::Random;

    // Helper function for common test setup
    fn create_test_grid() -> Grid<i32> {
        Grid::builder(3, 3, vec2(300.0, 300.0))
            .with_cell_data(|row, col, _bounds, _rng| {
                (row * 3 + col) as i32  // Simple test data
            })
            .build(&mut Random::default())
    }

    #[test]
    fn test_grid_creation_dimensions() {
        let grid = create_test_grid();
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 3);
        assert_eq!(grid.total_cells(), 9);
    }

    #[test]
    fn test_norm_at_center() {
        let grid = create_test_grid();
        let cell = grid.get(1, 1).unwrap();  // Middle cell

        let abs_pos = cell.norm(vec2(0.5, 0.5));

        // Middle cell offset is (100, 100), center is at +50,+50
        assert_eq!(abs_pos, vec2(150.0, 150.0));
    }

    // ... more tests
}
```

### Testing Benefits

1. **Fast Development Iteration** - Run `cargo test` instead of launching sketches
2. **Coordinate Math Validation** - Ensure all transformations are correct
3. **Regression Prevention** - Changes won't break existing functionality
4. **Documentation** - Tests show how the API is meant to be used
5. **Edge Case Coverage** - Test boundary conditions systematically
6. **TDD-Friendly** - Can write tests first, implement to pass them

### Running Tests

```bash
# Run all tests
cargo test

# Run only gridutils tests
cargo test gridutils

# Run specific test
cargo test test_norm_at_center

# Run with output
cargo test -- --nocapture
```

## Implementation Steps

1. Create `notan_sketches/src/gridutils.rs` with core types
2. Implement `Grid<T>` struct with storage and metadata
3. Implement `GridBuilder<T>` with builder pattern
4. Implement `CellContext<T>` and `CellContextMut<T>` with ALL coordinate helpers:
   - Normalized coordinate methods (PRIMARY): `norm()`, `norm_abs()`, `to_norm()`, etc.
   - Pixel coordinate methods (SECONDARY): `abs()`, `to_local()`, `center()`, etc.
   - Canvas-wide helpers on Grid: `norm_to_pixels()`, `pixels_to_norm()`
5. Implement iteration methods (`cells()`, `cells_mut()`)
6. Implement random access (`get()`, `get_mut()`)
7. Implement bulk operations (`regenerate_cells()`)
8. Implement debug helpers (`draw_overlay()`)
9. **Write comprehensive tests following the Testing Strategy above**
10. Add module to `lib.rs`
11. Write documentation emphasizing normalized coordinates as primary approach
12. (Optional) Refactor `claudes_first.rs` as demonstration

## Success Criteria

A sketch using `Grid` should:
- Reduce boilerplate by ~50-80 lines
- Eliminate manual index tracking
- Provide clearer, more declarative code
- Support both simple and complex use cases
- Work seamlessly in `init()`, `update()`, and `draw()` functions
- Have zero/minimal performance overhead vs manual implementation
- Enable resolution-independent designs through normalized coordinates
- Make sketches portable across different work sizes without code changes

## Example: Before and After

### Before (Current claudes_first.rs pattern)
```rust
const ROWS: u32 = 10;
const COLS: u32 = 10;

struct State {
    tile_width: f32,
    tile_height: f32,
    circle_positions: Vec<Vec2>,
    circle_colors: Vec<Color>,
    // ... more parallel arrays
}

fn init() {
    let tile_width = work_size.x / COLS as f32;
    let tile_height = work_size.y / ROWS as f32;

    let mut circle_positions = Vec::new();
    let mut circle_colors = Vec::new();

    for _ in 0..(ROWS * COLS) {
        circle_positions.push(vec2(/* ... */));
        circle_colors.push(/* ... */);
    }

    State { tile_width, tile_height, circle_positions, circle_colors }
}

fn draw() {
    let mut tile_index = 0;
    for row in 0..ROWS {
        for col in 0..COLS {
            let tile_x = col as f32 * state.tile_width;
            let tile_y = row as f32 * state.tile_height;
            let circle_pos = state.circle_positions[tile_index];
            let abs_x = tile_x + circle_pos.x;
            let abs_y = tile_y + circle_pos.y;

            draw.circle(radius)
                .position(abs_x, abs_y)
                .color(state.circle_colors[tile_index]);

            tile_index += 1;
        }
    }
}
```

### After (With Grid utilities + Normalized Coordinates)
```rust
struct CellData {
    position: Vec2,  // Stored as normalized 0.0-1.0 (resolution-independent!)
    color: Color,
}

struct State {
    grid: Grid<CellData>,
}

fn init() {
    let grid = Grid::builder(10, 10, work_size)
        .with_cell_data(|row, col, bounds, rng| {
            CellData {
                // Work in normalized space - always 0.0 to 1.0
                position: vec2(
                    rng.gen_range(0.1..0.9),  // 10% margin
                    rng.gen_range(0.1..0.9),
                ),
                color: palette.choose(rng),
            }
        })
        .build(&mut rng);

    State { grid }
}

fn draw() {
    for cell in state.grid.cells() {
        // Convert normalized (0-1) to absolute pixels for drawing
        let abs_pos = cell.norm(cell.data.position);  // Short & sweet!

        draw.circle(radius)
            .position(abs_pos.x, abs_pos.y)
            .color(cell.data.color);
    }
}

// BONUS: This sketch now works at ANY work_size without changes!
```

## Design Principles

- **Resolution-independent first**: Normalized coordinates (0-1) as primary approach, pixel coords as fallback
- **Zero-cost abstractions**: Iterator-based design compiles to same code as manual loops
- **Ergonomic API**: Short method names (`norm()`, `abs()`), reduce cognitive load, make intent clear
- **Flexible**: Support stateless (empty `()` data) and stateful use cases
- **Composable**: Works with existing notan types (Vec2, Color, Draw, Random, etc.)
- **Safe**: Leverage Rust's ownership system to prevent index-out-of-bounds errors
- **Consistent**: Follow patterns from existing modules (utils.rs, shaderutils.rs)
- **Future-proof**: Sketches work across different work sizes without modification
