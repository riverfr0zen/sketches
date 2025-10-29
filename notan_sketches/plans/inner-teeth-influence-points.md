# Influence Points System for Inner-Teeth Sketch

## Goal

Add a distance-based influence system to the `inner-teeth` sketch where randomly positioned "influence points" modulate the visual properties of cells based on their proximity. Initially, this will affect tooth height, with closer cells having taller teeth.

## Motivation

Currently, all teeth in the inner-teeth sketch are generated with uniform randomness within each cell. This creates visual interest at the cell level, but the overall composition lacks larger-scale structure or flow. By introducing influence points that affect nearby cells differently than distant ones, we can create:

- **Emergent patterns** - Areas of concentration and variation across the canvas
- **Visual hierarchy** - Some regions become focal points with distinctive characteristics
- **Organic cohesion** - Related cells respond similarly to shared influences
- **Dynamic regeneration** - Each regeneration creates a unique influence map

This approach is common in generative art, appearing in techniques like:
- Distance field effects
- Worley noise (cell noise)
- Circle packing algorithms
- Flow field generation

## Current State Analysis

From `inner-teeth.rs` (lines 120-172), the `generate_cell_data()` function:
- Takes `bounds: Rect` and `rng: &mut Random` as parameters
- Generates teeth with `max_height = 0.4`, `min_height = 0.10`
- Creates 8 teeth per side (32 total per cell)
- Stores teeth in normalized coordinates (0.0-1.0)
- Is called during grid initialization and regeneration

Key constants (lines 11-16):
```rust
const MAX_ROWS: u32 = 20;
const MAX_COLS: u32 = 20;
const GRID_STROKE: f32 = 5.0;
const GUMS_COLOR: Color = Color::new(0.9, 0.3, 0.3, 1.0);
const MOUTH_COLOR: Color = Color::new(0.7, 0.2, 0.1, 1.0);
const THROAT_COLOR: Color = Color::new(0.85, 0.27, 0.27, 1.0);
```

## Design Overview

### Influence Points

**Definition**: Random points distributed across the canvas that exert distance-based influence on cell properties.

**Count**: 1 influence point per 4 grid cells (rounded up)
- 4x4 grid (16 cells) → 4 influence points
- 10x10 grid (100 cells) → 25 influence points
- 20x20 grid (400 cells) → 100 influence points

**Placement**: Uniform random distribution in normalized canvas space (0.0-1.0, 0.0-1.0)

**Lifetime**: Regenerated whenever the grid is regenerated (R key press)

### Distance-Based Influence

**Approach**: For each cell, calculate distance from cell center to the nearest influence point.

**Coordinate System**: Use normalized canvas-wide coordinates (0.0-1.0) for both cell centers and influence points, ensuring resolution independence.

**Distance Metric**: Euclidean distance in normalized space:
```rust
let dx = cell_center_norm.x - influence_point.x;
let dy = cell_center_norm.y - influence_point.y;
let distance = (dx * dx + dy * dy).sqrt();
```

**Influence Mapping**: Distance → max_height adjustment
- Closer cells (smaller distance) → Taller teeth (higher max_height)
- Distant cells (larger distance) → Shorter teeth (lower max_height)

### Initial Implementation: Tooth Height Modulation

**Constants (Modified)**:
```rust
const BASE_MAX_HEIGHT: f32 = 0.05;  // Reduced from 0.4
const MIN_HEIGHT: f32 = 0.10;       // Unchanged
const INFLUENCE_RADIUS: f32 = 0.3;  // Max distance for influence (in normalized space)
const MAX_HEIGHT_BOOST: f32 = 0.35; // Maximum additional height from influence
```

**Mapping Function**: Linear interpolation based on distance
```rust
fn calculate_max_height(distance_to_nearest: f32) -> f32 {
    if distance_to_nearest >= INFLUENCE_RADIUS {
        BASE_MAX_HEIGHT
    } else {
        let influence_factor = 1.0 - (distance_to_nearest / INFLUENCE_RADIUS);
        BASE_MAX_HEIGHT + (MAX_HEIGHT_BOOST * influence_factor)
    }
}
```

This creates a gradient effect:
- At influence point (distance = 0.0): max_height = 0.05 + 0.35 = 0.40 (original max)
- At radius edge (distance = 0.3): max_height = 0.05 (base)
- Beyond radius (distance > 0.3): max_height = 0.05 (base)

## Data Structure Changes

### State Modifications

**Add to State struct** (line 40):
```rust
#[derive(AppState)]
struct State {
    rng: Random,
    work_size: Vec2,
    grid: Grid<CellData>,
    palette: PalettesSelection,
    show_grid: bool,
    needs_redraw: bool,
    draw: Draw,
    influence_points: Vec<Vec2>,  // NEW: Stored in normalized canvas coords (0-1)
}
```

### Helper Functions

**Calculate number of influence points**:
```rust
fn calculate_influence_point_count(total_cells: usize) -> usize {
    ((total_cells as f32 / 4.0).ceil() as usize).max(1)
}
```

**Generate influence points**:
```rust
fn generate_influence_points(count: usize, rng: &mut Random) -> Vec<Vec2> {
    (0..count)
        .map(|_| vec2(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)))
        .collect()
}
```

**Find nearest influence point distance**:
```rust
fn distance_to_nearest_influence(
    cell_center_norm: Vec2,
    influence_points: &[Vec2],
) -> f32 {
    influence_points
        .iter()
        .map(|&point| {
            let dx = cell_center_norm.x - point.x;
            let dy = cell_center_norm.y - point.y;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(f32::INFINITY, f32::min)
}
```

**Calculate max height from distance**:
```rust
const BASE_MAX_HEIGHT: f32 = 0.05;
const INFLUENCE_RADIUS: f32 = 0.3;
const MAX_HEIGHT_BOOST: f32 = 0.35;

fn calculate_max_height_from_influence(distance: f32) -> f32 {
    if distance >= INFLUENCE_RADIUS {
        BASE_MAX_HEIGHT
    } else {
        let influence_factor = 1.0 - (distance / INFLUENCE_RADIUS);
        BASE_MAX_HEIGHT + (MAX_HEIGHT_BOOST * influence_factor)
    }
}
```

## Implementation Steps

### Step 1: Add Constants and Helper Functions

**Location**: After existing constants (after line 21)

```rust
const BASE_MAX_HEIGHT: f32 = 0.05;  // New base for teeth
const INFLUENCE_RADIUS: f32 = 0.3;  // Radius of influence in normalized space
const MAX_HEIGHT_BOOST: f32 = 0.35; // Max boost from influence

fn calculate_influence_point_count(total_cells: usize) -> usize {
    ((total_cells as f32 / 4.0).ceil() as usize).max(1)
}

fn generate_influence_points(count: usize, rng: &mut Random) -> Vec<Vec2> {
    (0..count)
        .map(|_| vec2(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)))
        .collect()
}

fn distance_to_nearest_influence(
    cell_center_norm: Vec2,
    influence_points: &[Vec2],
) -> f32 {
    influence_points
        .iter()
        .map(|&point| {
            let dx = cell_center_norm.x - point.x;
            let dy = cell_center_norm.y - point.y;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(f32::INFINITY, f32::min)
}

fn calculate_max_height_from_influence(distance: f32) -> f32 {
    if distance >= INFLUENCE_RADIUS {
        BASE_MAX_HEIGHT
    } else {
        let influence_factor = 1.0 - (distance / INFLUENCE_RADIUS);
        BASE_MAX_HEIGHT + (MAX_HEIGHT_BOOST * influence_factor)
    }
}
```

### Step 2: Modify State Struct

**Location**: Line 40-49

**Before**:
```rust
#[derive(AppState)]
struct State {
    rng: Random,
    work_size: Vec2,
    grid: Grid<CellData>,
    palette: PalettesSelection,
    show_grid: bool,
    needs_redraw: bool,
    draw: Draw,
}
```

**After**:
```rust
#[derive(AppState)]
struct State {
    rng: Random,
    work_size: Vec2,
    grid: Grid<CellData>,
    palette: PalettesSelection,
    show_grid: bool,
    needs_redraw: bool,
    draw: Draw,
    influence_points: Vec<Vec2>,  // NEW
}
```

### Step 3: Modify `generate_cell_data()` Function

**Challenge**: The current signature is:
```rust
fn generate_cell_data(_bounds: Rect, rng: &mut Random) -> CellData
```

But we need access to:
1. Cell position (to calculate center in normalized canvas coords)
2. Influence points (to calculate distance)

**Solution**: The Grid builder's `with_cell_data` closure provides row/col, and the Grid knows work_size. We need to either:
- Pass influence points through somehow, OR
- Restructure to generate cells differently

**Approach A** (Cleaner): Pass influence points via closure capture

```rust
// In init() and update() where grid is created
let influence_points = generate_influence_points(count, &mut rng);
let grid = Grid::builder(rows, cols, work_size)
    .with_cell_data(|row, col, bounds, rng| {
        // Calculate cell center in normalized canvas space
        let cell_width = work_size.x / cols as f32;
        let cell_height = work_size.y / rows as f32;
        let cell_offset_x = col as f32 * cell_width;
        let cell_offset_y = row as f32 * cell_height;
        let cell_center_abs = vec2(
            cell_offset_x + cell_width * 0.5,
            cell_offset_y + cell_height * 0.5,
        );
        let cell_center_norm = vec2(
            cell_center_abs.x / work_size.x,
            cell_center_abs.y / work_size.y,
        );

        let distance = distance_to_nearest_influence(cell_center_norm, &influence_points);
        let max_height = calculate_max_height_from_influence(distance);

        generate_cell_data_with_max_height(bounds, rng, max_height)
    })
    .build(&mut rng);
```

**Approach B** (More organized): Create a new function that encapsulates the logic

```rust
fn generate_cell_data_influenced(
    _row: u32,
    _col: u32,
    bounds: Rect,
    cell_center_norm: Vec2,
    influence_points: &[Vec2],
    rng: &mut Random,
) -> CellData {
    let distance = distance_to_nearest_influence(cell_center_norm, influence_points);
    let max_height = calculate_max_height_from_influence(distance);

    // Generate teeth with calculated max_height
    let mut teeth: Vec<Tooth> = vec![];
    let min_height = 0.10;
    let tooth_width = 0.1;
    let padding = 0.05;

    for i in 2..10 {
        // Bottom teeth
        let tooth_height = rng.gen_range(min_height..max_height);
        // ... rest of tooth generation ...
    }

    // Throat details (unchanged)
    let throat_center = vec2(0.5, 0.5);
    let throat_radius = vec2(0.25, 0.25);

    CellData {
        teeth,
        throat_center,
        throat_radius,
    }
}
```

**Recommended**: Use Approach B for clarity. The Grid builder call becomes:

```rust
let grid = Grid::builder(rows, cols, work_size)
    .with_cell_data(|row, col, bounds, rng| {
        // Calculate cell center in normalized canvas space
        let cell_center_norm = calculate_cell_center_norm(row, col, rows, cols);
        generate_cell_data_influenced(
            row,
            col,
            bounds,
            cell_center_norm,
            &influence_points,
            rng,
        )
    })
    .build(&mut rng);
```

With helper:
```rust
fn calculate_cell_center_norm(row: u32, col: u32, total_rows: u32, total_cols: u32) -> Vec2 {
    vec2(
        (col as f32 + 0.5) / total_cols as f32,
        (row as f32 + 0.5) / total_rows as f32,
    )
}
```

### Step 4: Update `init()` Function

**Location**: Lines 51-85

**Before**:
```rust
fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let work_size = get_work_size_for_screen(app, gfx);
    log::info!("Work size: {:?}", work_size);

    let palette: PalettesSelection = rng.gen();
    log::info!("Palette: {:?}", palette);

    let rows = rng.gen_range(1..MAX_ROWS);
    let cols = rng.gen_range(1..MAX_COLS);

    let grid = Grid::builder(rows, cols, work_size)
        .with_cell_data(|_row, _col, bounds, rng| generate_cell_data(bounds, rng))
        .build(&mut rng);

    log::info!("Created {}x{} grid", rows, cols);
    log::info!("Press R to regenerate with new palette");
    log::info!("Press G to toggle grid overlay");

    let draw = get_draw_setup(gfx, work_size, false, BG_COLOR);

    State {
        rng,
        work_size,
        grid,
        palette,
        show_grid: false,
        needs_redraw: true,
        draw,
    }
}
```

**After**:
```rust
fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let (mut rng, seed) = get_rng(None);
    log::info!("Seed: {}", seed);

    let work_size = get_work_size_for_screen(app, gfx);
    log::info!("Work size: {:?}", work_size);

    let palette: PalettesSelection = rng.gen();
    log::info!("Palette: {:?}", palette);

    let rows = rng.gen_range(1..MAX_ROWS);
    let cols = rng.gen_range(1..MAX_COLS);

    // NEW: Generate influence points
    let total_cells = (rows * cols) as usize;
    let influence_count = calculate_influence_point_count(total_cells);
    let influence_points = generate_influence_points(influence_count, &mut rng);

    log::info!("Created {} influence points for {}x{} grid", influence_count, rows, cols);

    // Modified: Grid generation with influence points
    let grid = Grid::builder(rows, cols, work_size)
        .with_cell_data(|row, col, bounds, rng| {
            let cell_center_norm = calculate_cell_center_norm(row, col, rows, cols);
            generate_cell_data_influenced(
                row,
                col,
                bounds,
                cell_center_norm,
                &influence_points,
                rng,
            )
        })
        .build(&mut rng);

    log::info!("Created {}x{} grid", rows, cols);
    log::info!("Press R to regenerate with new palette");
    log::info!("Press G to toggle grid overlay");

    let draw = get_draw_setup(gfx, work_size, false, BG_COLOR);

    State {
        rng,
        work_size,
        grid,
        palette,
        show_grid: false,
        needs_redraw: true,
        draw,
        influence_points,  // NEW
    }
}
```

### Step 5: Update `update()` Function

**Location**: Lines 87-117

Need to regenerate influence points when R key is pressed.

**Before** (lines 98-106):
```rust
// Create a new grid with different size
let rows = state.rng.gen_range(1..MAX_ROWS);
let cols = state.rng.gen_range(1..MAX_COLS);

state.grid = Grid::builder(rows, cols, state.work_size)
    .with_cell_data(|_row, _col, bounds, rng| generate_cell_data(bounds, rng))
    .build(&mut state.rng);

log::info!("Created {}x{} grid", rows, cols);
```

**After**:
```rust
// Create a new grid with different size
let rows = state.rng.gen_range(1..MAX_ROWS);
let cols = state.rng.gen_range(1..MAX_COLS);

// NEW: Generate new influence points
let total_cells = (rows * cols) as usize;
let influence_count = calculate_influence_point_count(total_cells);
state.influence_points = generate_influence_points(influence_count, &mut state.rng);

log::info!("Created {} influence points for {}x{} grid", influence_count, rows, cols);

// Modified: Grid generation with influence points
state.grid = Grid::builder(rows, cols, state.work_size)
    .with_cell_data(|row, col, bounds, rng| {
        let cell_center_norm = calculate_cell_center_norm(row, col, rows, cols);
        generate_cell_data_influenced(
            row,
            col,
            bounds,
            cell_center_norm,
            &state.influence_points,
            rng,
        )
    })
    .build(&mut state.rng);

log::info!("Created {}x{} grid", rows, cols);
```

### Step 6: (Optional) Visualize Influence Points

Add a debug visualization to see influence points when grid overlay is enabled.

**Add to `draw()` function** (after line 243, before grid overlay):

```rust
// Optional: Draw influence points for debugging
if state.show_grid {
    for &point in &state.influence_points {
        let abs_pos = state.grid.norm_to_pixels(point);
        state
            .draw
            .circle(10.0)
            .position(abs_pos.x, abs_pos.y)
            .color(Color::YELLOW)
            .fill();

        // Draw influence radius
        state
            .draw
            .circle(INFLUENCE_RADIUS * state.work_size.x.min(state.work_size.y))
            .position(abs_pos.x, abs_pos.y)
            .color(Color::from_rgba(255, 255, 0, 50))
            .stroke(2.0);
    }
}
```

## Code Summary: Complete Modified Sections

### New Constants (after line 21)
```rust
const BASE_MAX_HEIGHT: f32 = 0.05;
const INFLUENCE_RADIUS: f32 = 0.3;
const MAX_HEIGHT_BOOST: f32 = 0.35;
```

### New Helper Functions (after constants)
```rust
fn calculate_influence_point_count(total_cells: usize) -> usize {
    ((total_cells as f32 / 4.0).ceil() as usize).max(1)
}

fn generate_influence_points(count: usize, rng: &mut Random) -> Vec<Vec2> {
    (0..count)
        .map(|_| vec2(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)))
        .collect()
}

fn distance_to_nearest_influence(
    cell_center_norm: Vec2,
    influence_points: &[Vec2],
) -> f32 {
    influence_points
        .iter()
        .map(|&point| {
            let dx = cell_center_norm.x - point.x;
            let dy = cell_center_norm.y - point.y;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(f32::INFINITY, f32::min)
}

fn calculate_max_height_from_influence(distance: f32) -> f32 {
    if distance >= INFLUENCE_RADIUS {
        BASE_MAX_HEIGHT
    } else {
        let influence_factor = 1.0 - (distance / INFLUENCE_RADIUS);
        BASE_MAX_HEIGHT + (MAX_HEIGHT_BOOST * influence_factor)
    }
}

fn calculate_cell_center_norm(row: u32, col: u32, total_rows: u32, total_cols: u32) -> Vec2 {
    vec2(
        (col as f32 + 0.5) / total_cols as f32,
        (row as f32 + 0.5) / total_rows as f32,
    )
}
```

### Modified `generate_cell_data()` (rename and add parameters)
```rust
fn generate_cell_data_influenced(
    _row: u32,
    _col: u32,
    _bounds: Rect,
    cell_center_norm: Vec2,
    influence_points: &[Vec2],
    rng: &mut Random,
) -> CellData {
    let distance = distance_to_nearest_influence(cell_center_norm, influence_points);
    let max_height = calculate_max_height_from_influence(distance);

    let mut teeth: Vec<Tooth> = vec![];
    let min_height = 0.10;
    let tooth_width = 0.1;
    let padding = 0.05;

    for i in 2..10 {
        let tooth_height = rng.gen_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(boundary - 0.05, 1.0 - tooth_height);
        let start = vec2(boundary - tooth_width, 1.0 - padding);
        let end = vec2(boundary, 1.0 - padding);
        teeth.push(Tooth { start, mid, end });

        let tooth_height = rng.gen_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(boundary - 0.05, tooth_height);
        let start = vec2(boundary - tooth_width, padding);
        let end = vec2(boundary, padding);
        teeth.push(Tooth { start, mid, end });

        let tooth_height = rng.gen_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(tooth_height, boundary - 0.05);
        let start = vec2(padding, boundary - tooth_width);
        let end = vec2(padding, boundary);
        teeth.push(Tooth { start, mid, end });

        let tooth_height = rng.gen_range(min_height..max_height);
        let boundary: f32 = i as f32 / 10.0;
        let mid = vec2(1.0 - tooth_height, boundary - 0.05);
        let start = vec2(1.0 - padding, boundary - tooth_width);
        let end = vec2(1.0 - padding, boundary);
        teeth.push(Tooth { start, mid, end });
    }

    let throat_center = vec2(0.5, 0.5);
    let throat_radius = vec2(0.25, 0.25);

    CellData {
        teeth,
        throat_center,
        throat_radius,
    }
}
```

## Testing Strategy

### Visual Tests
1. **Baseline**: Run sketch without changes, screenshot
2. **With Influence**: Run with influence points, verify:
   - Teeth near influence points are taller
   - Teeth far from influence points are shorter (base height)
   - Gradient effect is smooth
3. **Grid Overlay**: Enable grid overlay (G key), verify:
   - Influence points are visible (if visualization added)
   - Influence radii are shown
   - Visual correlation between influence points and tooth height
4. **Regeneration**: Press R multiple times, verify:
   - Influence points change positions
   - Tooth height patterns change accordingly
   - No crashes or visual glitches

### Edge Cases
1. **Single cell grid**: 1x1 → 1 influence point
2. **Small grid**: 2x2 → 1 influence point
3. **Large grid**: 20x20 → 100 influence points
4. **Extreme influence**: All influence points clustered in one corner
5. **No overlap**: Influence radii don't overlap at all

### Manual Verification
1. Check log output for influence point count
2. Verify tooth height variation matches expected pattern
3. Confirm influence points regenerate on R press
4. Check performance remains smooth (60fps)

## Future Extensions

### 1. Color Influence
Modulate cell colors (gums, mouth, throat, teeth) based on distance to influence points:
```rust
fn calculate_color_from_influence(
    base_color: Color,
    alt_color: Color,
    distance: f32,
) -> Color {
    let influence_factor = calculate_influence_factor(distance);
    Color::new(
        base_color.r + (alt_color.r - base_color.r) * influence_factor,
        base_color.g + (alt_color.g - base_color.g) * influence_factor,
        base_color.b + (alt_color.b - base_color.b) * influence_factor,
        1.0,
    )
}
```

### 2. Throat Size Influence
Vary throat ellipse size based on distance:
```rust
let throat_radius_factor = 1.0 + (influence_factor * 0.5); // Up to 50% larger
let throat_radius = vec2(0.25 * throat_radius_factor, 0.25 * throat_radius_factor);
```

### 3. Tooth Count Influence
Generate more teeth in cells closer to influence points:
```rust
let tooth_count = if influence_factor > 0.7 { 12 } else { 8 };
for i in 2..(2 + tooth_count) { ... }
```

### 4. Multiple Influence Types
Have different types of influence points with different effects:
```rust
enum InfluenceType {
    HeightBoost,    // Makes teeth taller
    ColorShift,     // Changes colors
    Density,        // Affects tooth count
    Animation,      // Triggers animation (future)
}

struct InfluencePoint {
    position: Vec2,
    influence_type: InfluenceType,
    strength: f32,
}
```

### 5. Animated Influence Points
Make influence points move slowly over time:
```rust
// In update()
for point in &mut state.influence_points {
    point.x += rng.gen_range(-0.001..0.001);
    point.y += rng.gen_range(-0.001..0.001);
    point.x = point.x.clamp(0.0, 1.0);
    point.y = point.y.clamp(0.0, 1.0);
}
state.grid.regenerate_cells(/* ... */); // Recalculate teeth
state.needs_redraw = true;
```

### 6. User-Controlled Influence Points
Allow clicking to add/remove influence points:
```rust
// In update()
if app.mouse.was_pressed(MouseButton::Left) {
    let mouse_pos = app.mouse.position();
    let norm_pos = state.grid.pixels_to_norm(vec2(mouse_pos.0, mouse_pos.1));
    state.influence_points.push(norm_pos);
    // Regenerate affected cells
}
```

### 7. Influence Decay Animation
Gradually reduce influence strength over time:
```rust
struct InfluencePoint {
    position: Vec2,
    strength: f32,  // 0.0 to 1.0
    age: f32,       // seconds since creation
}

// In update()
for point in &mut state.influence_points {
    point.age += app.timer.delta_f32();
    point.strength = (1.0 - point.age / 10.0).max(0.0); // Decay over 10 seconds
}
```

### 8. Distance Field Visualization
Add a shader-based distance field overlay showing influence gradient:
```rust
// Fragment shader
uniform vec2 influence_points[100];
uniform int influence_count;

float min_distance = 999.0;
for (int i = 0; i < influence_count; i++) {
    float d = distance(v_uv, influence_points[i]);
    min_distance = min(min_distance, d);
}

float influence = 1.0 - clamp(min_distance / INFLUENCE_RADIUS, 0.0, 1.0);
color = mix(base_color, highlight_color, influence);
```

### 9. Voronoi Cells
Assign each grid cell to its nearest influence point and apply uniform properties:
```rust
fn find_nearest_influence_index(
    cell_center: Vec2,
    influence_points: &[Vec2],
) -> usize {
    influence_points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            let da = distance(cell_center, **a);
            let db = distance(cell_center, **b);
            da.partial_cmp(&db).unwrap()
        })
        .map(|(idx, _)| idx)
        .unwrap()
}

// Assign colors to influence points
let influence_colors: Vec<Color> = (0..influence_count)
    .map(|_| palette.choose(&mut rng))
    .collect();

// In cell generation
let nearest_idx = find_nearest_influence_index(cell_center_norm, &influence_points);
let cell_color = influence_colors[nearest_idx];
```

### 10. Flow Field Integration
Use influence points as sources/sinks in a flow field for particle animation:
```rust
// Calculate flow direction at any point
fn calculate_flow_direction(pos: Vec2, influence_points: &[Vec2]) -> Vec2 {
    let mut flow = vec2(0.0, 0.0);
    for &point in influence_points {
        let dir = point - pos;
        let dist = dir.length();
        if dist > 0.001 {
            flow += dir.normalize() / (dist + 0.1); // Inverse distance weighting
        }
    }
    flow.normalize()
}
```

## Success Criteria

The implementation is successful when:

1. **Visual Impact**
   - Clear variation in tooth height across the canvas
   - Visible "hotspots" where teeth are taller near influence points
   - Smooth gradient effect from influenced to base areas
   - Overall composition has more structure than pure random

2. **Technical Correctness**
   - Distance calculations use normalized coordinates correctly
   - Base max_height is 0.05 as specified
   - Influence points count matches expected (total_cells / 4, rounded up)
   - Teeth heights stay within valid range (min_height to calculated max_height)

3. **Regeneration Behavior**
   - R key regenerates both grid and influence points
   - New patterns emerge with each regeneration
   - No memory leaks or performance degradation over multiple regenerations

4. **Code Quality**
   - Helper functions are clear and well-named
   - No duplicate logic
   - Consistent with existing codebase style
   - Commented where non-obvious

5. **Performance**
   - Maintains 60fps at 20x20 grid with 100 influence points
   - Distance calculations don't cause noticeable lag
   - Grid regeneration remains fast (<100ms)

6. **Extensibility**
   - Easy to adjust influence parameters (radius, boost amount)
   - Easy to add other influenced properties (color, throat size)
   - Clear structure for future multi-influence-type system

## Implementation Checklist

- [ ] Add new constants (BASE_MAX_HEIGHT, INFLUENCE_RADIUS, MAX_HEIGHT_BOOST)
- [ ] Implement `calculate_influence_point_count()`
- [ ] Implement `generate_influence_points()`
- [ ] Implement `distance_to_nearest_influence()`
- [ ] Implement `calculate_max_height_from_influence()`
- [ ] Implement `calculate_cell_center_norm()`
- [ ] Add `influence_points: Vec<Vec2>` to State struct
- [ ] Rename `generate_cell_data()` to `generate_cell_data_influenced()`
- [ ] Add parameters to `generate_cell_data_influenced()`
- [ ] Update `init()` to generate and use influence points
- [ ] Update `update()` to regenerate influence points on R key
- [ ] (Optional) Add influence point visualization in `draw()`
- [ ] Test with small grid (2x2)
- [ ] Test with medium grid (10x10)
- [ ] Test with large grid (20x20)
- [ ] Test regeneration multiple times
- [ ] Verify performance remains good
- [ ] Take before/after screenshots for documentation

## References

- **Grid utilities**: See `notan_sketches/plans/grid.md` for coordinate system details
- **Inner-teeth sketch**: `notan_sketches/examples/inner-teeth.rs`
- **Distance field techniques**: Used in Worley noise, circle packing, flow fields
- **Normalized coordinates**: All calculations use 0.0-1.0 canvas space for resolution independence

## Notes

- This is phase 1 (tooth height only). Future phases can add color, throat size, tooth count, etc.
- The influence radius (0.3 in normalized space) means roughly 30% of canvas width/height
- Linear interpolation is simple but effective; could explore other curves (ease-in/out, exponential)
- Consider adding keyboard shortcuts to adjust influence parameters in real-time (future enhancement)
- The 1:4 ratio (1 influence point per 4 cells) is arbitrary; could be made configurable
