# Grid Utilities Module

The `gridutils` module provides utilities for creating grid-based generative art sketches with minimal boilerplate.

## Location

- **Module**: `notan_sketches/src/gridutils.rs`
- **Tests**: `notan_sketches/tests/gridutils_test.rs`
- **Examples**:
  - `notan_sketches/examples/grid_demo.rs` - Simple demonstration without shaders
  - `notan_sketches/examples/claudes_first_grid.rs` - Full-featured example with shaders

## Quick Start

```rust
use notan_sketches::gridutils::Grid;

// In init()
let grid = Grid::builder(ROWS, COLS, work_size)
    .with_cell_data(|row, col, bounds, rng| {
        CellData {
            position: vec2(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
            color: colors::Palettes::choose_color(&palette),
        }
    })
    .build(&mut rng);

// In draw()
for cell in grid.cells() {
    let abs_pos = cell.to_px(cell.data.position);
    draw.circle(radius).position(abs_pos.x, abs_pos.y).color(cell.data.color);
}
```

## Key Concepts

### Normalized Coordinates (0.0-1.0)

Store all positions as normalized coordinates (0-1 scale) within each cell. This makes sketches resolution-independent.

```rust
#[derive(Clone)]
struct CellData {
    position: Vec2,  // 0.0-1.0 within the cell
    size: Vec2,      // 0.0-1.0 scale
    color: Color,
}
```

### Coordinate Transformation Methods

#### Primary Methods (Most Common)

- **`to_px(norm_local: Vec2) -> Vec2`** - Convert cell-local normalized (0-1) to absolute pixel coordinates
  - This is the main method you'll use for drawing
  - Example: `let abs_pos = cell.to_px(cell.data.position);`

- **`norm_size(norm_size: Vec2) -> Vec2`** - Convert normalized size to absolute pixel size
  - For sizes/dimensions, not positions (doesn't add cell offset)
  - Example: `let rect_size = cell.norm_size(vec2(0.5, 0.3));`

#### Secondary Methods (Less Common)

- **`to_canvas_norm(norm_local: Vec2) -> Vec2`** - Convert cell-local normalized to canvas-wide normalized
  - Useful for shader uniforms or cross-cell effects

- **`to_norm(abs_pixels: Vec2) -> Vec2`** - Convert absolute pixels to cell-local normalized
  - Useful for mouse/touch input handling

- **`abs(local_pixels: Vec2) -> Vec2`** - Convert cell-local pixels to absolute pixels
- **`to_local(abs_pixels: Vec2) -> Vec2`** - Convert absolute pixels to cell-local pixels
- **`center() -> Vec2`** - Get cell center in absolute pixels
- **`center_norm() -> Vec2`** - Get cell center in normalized coords (always 0.5, 0.5)

## Common Patterns

### Cell Iteration (Immutable)

```rust
for cell in state.grid.cells() {
    let pos = cell.to_px(cell.data.position);
    draw.circle(radius).position(pos.x, pos.y).color(cell.data.color);
}
```

### Cell Iteration (Mutable)

```rust
for mut cell in state.grid.cells_mut() {
    cell.data.position += velocity * dt;
    cell.data.position = cell.data.position.clamp(Vec2::ZERO, Vec2::ONE);
}
```

### Regenerating All Cells

```rust
state.grid.regenerate_cells(&mut state.rng, |row, col, bounds, rng| {
    generate_cell_data(row, col, bounds, rng, &state.palette)
});
```

### Grid Overlay (Debug)

```rust
if state.show_grid {
    state.grid.draw_overlay(&mut draw, Color::BLACK, 2.0);
}
```

### Random Access

```rust
if let Some(cell) = state.grid.get(row, col) {
    // Work with specific cell
}
```

## Architecture

### Grid<T>

Owns all cell data in a single structure. Lives in AppState.

- **Type parameter T**: Your custom cell data type
- **Storage**: Flattened Vec in row-major order
- **Metadata**: Rows, cols, work_size, cell dimensions

### CellContext<'a, T> / CellContextMut<'a, T>

Yielded by iteration methods. Provides:
- Cell metadata (row, col, bounds, offset)
- Reference to cell data
- Coordinate transformation helpers

## Benefits Over Manual Grid Management

1. **No manual tile_index tracking**
2. **No manual offset calculations**
3. **Unified cell data** (single Grid<T> vs multiple parallel Vecs)
4. **Resolution independence** via normalized coordinates
5. **Built-in grid overlay** (1 line vs 20+ lines)
6. **One-line regeneration**
7. **Iterator-based API** (clean, idiomatic Rust)

## Testing

Run tests with:
```bash
cd notan_sketches
cargo test --test gridutils_test
```

31 comprehensive tests covering all functionality.

## Examples

- **grid_demo.rs** - Start here! Simple demonstration with rectangles and circles
- **claudes_first_grid.rs** - Advanced example with shaders and child circles

Run examples:
```bash
cd notan_sketches
cargo run --release --example grid_demo
```
