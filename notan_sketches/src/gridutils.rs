use notan::draw::*;
use notan::math::{Rect, Vec2};
use notan::prelude::{Color, Random};

/// A grid structure that manages a 2D array of cells with optional per-cell data.
///
/// The Grid owns both the grid structure (dimensions, cell sizes) and all per-cell data.
/// It's designed to be stored in AppState and used across init(), update(), and draw().
///
/// # Type Parameter
/// - `T`: The type of data stored in each cell. Use `()` for grids without data.
///
/// # Coordinate System
/// The grid supports four coordinate spaces:
/// 1. Cell-local normalized (0.0-1.0) - RECOMMENDED for resolution independence
/// 2. Cell-local pixels (0.0 to cell_width/height)
/// 3. Canvas-wide normalized (0.0-1.0)
/// 4. Canvas-wide pixels (absolute screen coordinates)
pub struct Grid<T> {
    rows: u32,
    cols: u32,
    work_size: Vec2,
    cell_width: f32,
    cell_height: f32,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    /// Create a builder for constructing a Grid with cell data.
    pub fn builder(rows: u32, cols: u32, work_size: Vec2) -> GridBuilder<T> {
        GridBuilder::new(rows, cols, work_size)
    }

    /// Get the number of rows in the grid.
    pub fn rows(&self) -> u32 {
        self.rows
    }

    /// Get the number of columns in the grid.
    pub fn cols(&self) -> u32 {
        self.cols
    }

    /// Get the width of each cell in pixels.
    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }

    /// Get the height of each cell in pixels.
    pub fn cell_height(&self) -> f32 {
        self.cell_height
    }

    /// Get the total number of cells in the grid.
    pub fn total_cells(&self) -> usize {
        self.cells.len()
    }

    /// Get the work size (total canvas dimensions).
    pub fn work_size(&self) -> Vec2 {
        self.work_size
    }

    /// Convert canvas-wide normalized coordinates (0-1) to absolute pixels.
    pub fn norm_to_pixels(&self, norm_pos: Vec2) -> Vec2 {
        Vec2::new(norm_pos.x * self.work_size.x, norm_pos.y * self.work_size.y)
    }

    /// Convert absolute pixel coordinates to canvas-wide normalized (0-1).
    pub fn pixels_to_norm(&self, abs_pos: Vec2) -> Vec2 {
        Vec2::new(abs_pos.x / self.work_size.x, abs_pos.y / self.work_size.y)
    }

    /// Convert row and column indices to a flat array index.
    fn index(&self, row: u32, col: u32) -> usize {
        (row * self.cols + col) as usize
    }

    /// Calculate the cell bounds (local rect) for a given cell.
    fn cell_bounds(&self) -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width: self.cell_width,
            height: self.cell_height,
        }
    }

    /// Calculate the offset (absolute position) for a cell's top-left corner.
    fn cell_offset(&self, row: u32, col: u32) -> Vec2 {
        Vec2::new(col as f32 * self.cell_width, row as f32 * self.cell_height)
    }
}

/// Builder for constructing a Grid with cell data.
pub struct GridBuilder<T> {
    rows: u32,
    cols: u32,
    work_size: Vec2,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> GridBuilder<T> {
    fn new(rows: u32, cols: u32, work_size: Vec2) -> Self {
        Self {
            rows,
            cols,
            work_size,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Generate cell data using a closure that receives cell metadata.
    ///
    /// The closure receives:
    /// - `row`: The row index (0-based)
    /// - `col`: The column index (0-based)
    /// - `bounds`: The cell's local rectangle (0, 0, cell_width, cell_height)
    /// - `rng`: A mutable reference to the random number generator
    ///
    /// # Example
    /// ```ignore
    /// let grid = Grid::builder(10, 10, work_size)
    ///     .with_cell_data(|row, col, bounds, rng| {
    ///         CellData {
    ///             position: vec2(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
    ///             color: palette.choose(rng),
    ///         }
    ///     })
    ///     .build(&mut rng);
    /// ```
    pub fn with_cell_data<F>(self, f: F) -> GridBuilderWithData<T, F>
    where
        F: Fn(u32, u32, Rect, &mut Random) -> T,
    {
        GridBuilderWithData {
            rows: self.rows,
            cols: self.cols,
            work_size: self.work_size,
            cell_data_fn: f,
        }
    }
}

/// Builder with a cell data generation function.
pub struct GridBuilderWithData<T, F>
where
    F: Fn(u32, u32, Rect, &mut Random) -> T,
{
    rows: u32,
    cols: u32,
    work_size: Vec2,
    cell_data_fn: F,
}

impl<T, F> GridBuilderWithData<T, F>
where
    F: Fn(u32, u32, Rect, &mut Random) -> T,
{
    /// Build the grid, generating cell data using the provided closure.
    pub fn build(self, rng: &mut Random) -> Grid<T> {
        let cell_width = self.work_size.x / self.cols as f32;
        let cell_height = self.work_size.y / self.rows as f32;
        let bounds = Rect {
            x: 0.0,
            y: 0.0,
            width: cell_width,
            height: cell_height,
        };

        let mut cells = Vec::with_capacity((self.rows * self.cols) as usize);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell_data = (self.cell_data_fn)(row, col, bounds, rng);
                cells.push(cell_data);
            }
        }

        Grid {
            rows: self.rows,
            cols: self.cols,
            work_size: self.work_size,
            cell_width,
            cell_height,
            cells,
        }
    }
}

/// Context for a cell during iteration, providing metadata and coordinate helpers.
///
/// This struct is yielded by the grid's iteration methods and provides:
/// - Cell position (row, col, index)
/// - Cell bounds and offset
/// - Reference to cell data
/// - Coordinate transformation helpers
pub struct CellContext<'a, T> {
    /// The row index of this cell (0-based).
    pub row: u32,
    /// The column index of this cell (0-based).
    pub col: u32,
    /// The cell's local rectangle (0, 0, cell_width, cell_height).
    pub bounds: Rect,
    /// The absolute position of the cell's top-left corner.
    pub offset: Vec2,
    /// Reference to the cell's data.
    pub data: &'a T,
    // Internal reference to parent grid for coordinate helpers
    work_size: Vec2,
}

impl<'a, T> CellContext<'a, T> {
    // ===== PRIMARY METHODS: Normalized Coordinates (RECOMMENDED) =====

    /// Convert cell-local normalized coordinates (0-1) to absolute pixel coordinates.
    ///
    /// This is the most commonly used method for drawing. It takes a position within
    /// the cell expressed as normalized coordinates (0.0 to 1.0) and converts it to
    /// absolute screen coordinates.
    ///
    /// # Example
    /// ```ignore
    /// for cell in grid.cells() {
    ///     let abs_pos = cell.norm(cell.data.position); // position stored as 0-1
    ///     draw.circle(radius).position(abs_pos.x, abs_pos.y);
    /// }
    /// ```
    pub fn norm(&self, norm_local: Vec2) -> Vec2 {
        self.offset
            + Vec2::new(
                norm_local.x * self.bounds.width,
                norm_local.y * self.bounds.height,
            )
    }

    /// Convert cell-local normalized coordinates (0-1) to canvas-wide normalized (0-1).
    ///
    /// Useful for cross-cell effects, shader uniforms, or working with canvas-wide
    /// normalized coordinates.
    pub fn norm_abs(&self, norm_local: Vec2) -> Vec2 {
        let abs_pixels = self.norm(norm_local);
        Vec2::new(
            abs_pixels.x / self.work_size.x,
            abs_pixels.y / self.work_size.y,
        )
    }

    /// Convert absolute pixel coordinates to cell-local normalized (0-1).
    ///
    /// Useful for converting mouse/touch positions to normalized cell coordinates.
    pub fn to_norm(&self, abs_pixels: Vec2) -> Vec2 {
        let local = abs_pixels - self.offset;
        Vec2::new(
            local.x / self.bounds.width,
            local.y / self.bounds.height,
        )
    }

    /// Convert canvas-wide normalized coordinates (0-1) to cell-local normalized (0-1).
    ///
    /// Useful for converting shader output or canvas-wide effects to cell coordinates.
    pub fn to_norm_local(&self, norm_abs: Vec2) -> Vec2 {
        let abs_pixels = Vec2::new(
            norm_abs.x * self.work_size.x,
            norm_abs.y * self.work_size.y,
        );
        self.to_norm(abs_pixels)
    }

    /// Get the center point of the cell in cell-local normalized coordinates.
    ///
    /// Always returns vec2(0.5, 0.5) for any cell.
    pub fn center_norm(&self) -> Vec2 {
        Vec2::new(0.5, 0.5)
    }

    /// Get the center point of the cell in canvas-wide normalized coordinates.
    pub fn center_norm_abs(&self) -> Vec2 {
        self.norm_abs(self.center_norm())
    }

    // ===== SECONDARY METHODS: Pixel Coordinates =====

    /// Convert cell-local pixel coordinates to absolute pixel coordinates.
    ///
    /// Use this if you're working with pixel-based positions within cells.
    pub fn abs(&self, local_pixels: Vec2) -> Vec2 {
        self.offset + local_pixels
    }

    /// Convert absolute pixel coordinates to cell-local pixel coordinates.
    pub fn to_local(&self, abs_pixels: Vec2) -> Vec2 {
        abs_pixels - self.offset
    }

    /// Get the center point of the cell in absolute pixel coordinates.
    pub fn center(&self) -> Vec2 {
        self.offset
            + Vec2::new(self.bounds.width * 0.5, self.bounds.height * 0.5)
    }

    // ===== METADATA =====

    /// Get the flattened index of this cell (row-major order).
    pub fn index(&self) -> usize {
        (self.row * (self.bounds.width / self.bounds.height) as u32 + self.col) as usize
    }
}

// ===== Iteration Methods =====

impl<T> Grid<T> {
    /// Iterate over all cells immutably.
    ///
    /// Returns an iterator that yields `CellContext` for each cell in row-major order.
    ///
    /// # Example
    /// ```ignore
    /// for cell in grid.cells() {
    ///     let pos = cell.norm(cell.data.position);
    ///     draw.circle(radius).position(pos.x, pos.y);
    /// }
    /// ```
    pub fn cells(&self) -> impl Iterator<Item = CellContext<'_, T>> {
        let cols = self.cols;
        let cell_width = self.cell_width;
        let cell_height = self.cell_height;
        let work_size = self.work_size;
        let bounds = self.cell_bounds();

        self.cells.iter().enumerate().map(move |(idx, data)| {
            let row = (idx as u32) / cols;
            let col = (idx as u32) % cols;
            let offset = Vec2::new(col as f32 * cell_width, row as f32 * cell_height);

            CellContext {
                row,
                col,
                bounds,
                offset,
                data,
                work_size,
            }
        })
    }

    /// Iterate over all cells mutably.
    ///
    /// Returns an iterator that yields `CellContextMut` for each cell in row-major order.
    ///
    /// # Example
    /// ```ignore
    /// for mut cell in grid.cells_mut() {
    ///     cell.data.position += velocity * dt;
    ///     cell.data.position.x = cell.data.position.x.clamp(0.0, 1.0);
    /// }
    /// ```
    pub fn cells_mut(&mut self) -> impl Iterator<Item = CellContextMut<'_, T>> {
        let cols = self.cols;
        let cell_width = self.cell_width;
        let cell_height = self.cell_height;
        let work_size = self.work_size;
        let bounds = self.cell_bounds();

        self.cells.iter_mut().enumerate().map(move |(idx, data)| {
            let row = (idx as u32) / cols;
            let col = (idx as u32) % cols;
            let offset = Vec2::new(col as f32 * cell_width, row as f32 * cell_height);

            CellContextMut {
                row,
                col,
                bounds,
                offset,
                data,
                work_size,
            }
        })
    }
}

// ===== Random Access Methods =====

impl<T> Grid<T> {
    /// Get an immutable reference to a specific cell by row and column.
    ///
    /// Returns `None` if the row or column is out of bounds.
    ///
    /// # Example
    /// ```ignore
    /// if let Some(cell) = grid.get(1, 2) {
    ///     println!("Cell at (1,2) has data: {:?}", cell.data);
    /// }
    /// ```
    pub fn get(&self, row: u32, col: u32) -> Option<CellContext<'_, T>> {
        if row >= self.rows || col >= self.cols {
            return None;
        }

        let idx = self.index(row, col);
        let bounds = self.cell_bounds();
        let offset = self.cell_offset(row, col);

        Some(CellContext {
            row,
            col,
            bounds,
            offset,
            data: &self.cells[idx],
            work_size: self.work_size,
        })
    }

    /// Get a mutable reference to a specific cell by row and column.
    ///
    /// Returns `None` if the row or column is out of bounds.
    ///
    /// # Example
    /// ```ignore
    /// if let Some(mut cell) = grid.get_mut(1, 2) {
    ///     cell.data.color = Color::RED;
    /// }
    /// ```
    pub fn get_mut(&mut self, row: u32, col: u32) -> Option<CellContextMut<'_, T>> {
        if row >= self.rows || col >= self.cols {
            return None;
        }

        let idx = self.index(row, col);
        let bounds = self.cell_bounds();
        let offset = self.cell_offset(row, col);
        let work_size = self.work_size;

        Some(CellContextMut {
            row,
            col,
            bounds,
            offset,
            data: &mut self.cells[idx],
            work_size,
        })
    }
}

// ===== Bulk Operations =====

impl<T> Grid<T> {
    /// Regenerate all cell data using a closure.
    ///
    /// This is useful for resetting or randomizing the grid, such as when the user
    /// presses 'R' to regenerate.
    ///
    /// # Example
    /// ```ignore
    /// grid.regenerate_cells(&mut rng, |row, col, bounds, rng| {
    ///     CellData {
    ///         position: vec2(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
    ///         color: palette.choose(rng),
    ///     }
    /// });
    /// ```
    pub fn regenerate_cells<F>(&mut self, rng: &mut Random, f: F)
    where
        F: Fn(u32, u32, Rect, &mut Random) -> T,
    {
        let bounds = self.cell_bounds();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = self.index(row, col);
                self.cells[idx] = f(row, col, bounds, rng);
            }
        }
    }
}

// ===== Debug Helpers =====

impl<T> Grid<T> {
    /// Draw grid overlay lines for debugging.
    ///
    /// Renders vertical and horizontal lines showing the grid structure.
    ///
    /// # Example
    /// ```ignore
    /// if state.show_grid {
    ///     state.grid.draw_overlay(&mut draw, Color::GREEN, 2.0);
    /// }
    /// ```
    pub fn draw_overlay(&self, draw: &mut Draw, color: Color, stroke_width: f32) {
        // Draw vertical lines
        for col in 0..=self.cols {
            let x = col as f32 * self.cell_width;
            draw.path()
                .move_to(x, 0.0)
                .line_to(x, self.work_size.y)
                .stroke_color(color)
                .stroke(stroke_width);
        }

        // Draw horizontal lines
        for row in 0..=self.rows {
            let y = row as f32 * self.cell_height;
            draw.path()
                .move_to(0.0, y)
                .line_to(self.work_size.x, y)
                .stroke_color(color)
                .stroke(stroke_width);
        }
    }
}

/// Mutable context for a cell during iteration.
///
/// Similar to CellContext but provides mutable access to cell data.
pub struct CellContextMut<'a, T> {
    /// The row index of this cell (0-based).
    pub row: u32,
    /// The column index of this cell (0-based).
    pub col: u32,
    /// The cell's local rectangle (0, 0, cell_width, cell_height).
    pub bounds: Rect,
    /// The absolute position of the cell's top-left corner.
    pub offset: Vec2,
    /// Mutable reference to the cell's data.
    pub data: &'a mut T,
    // Internal reference to parent grid for coordinate helpers
    work_size: Vec2,
}

impl<'a, T> CellContextMut<'a, T> {
    // Provide the same coordinate helper methods as CellContext

    /// Convert cell-local normalized coordinates (0-1) to absolute pixel coordinates.
    pub fn norm(&self, norm_local: Vec2) -> Vec2 {
        self.offset
            + Vec2::new(
                norm_local.x * self.bounds.width,
                norm_local.y * self.bounds.height,
            )
    }

    /// Convert cell-local normalized coordinates (0-1) to canvas-wide normalized (0-1).
    pub fn norm_abs(&self, norm_local: Vec2) -> Vec2 {
        let abs_pixels = self.norm(norm_local);
        Vec2::new(
            abs_pixels.x / self.work_size.x,
            abs_pixels.y / self.work_size.y,
        )
    }

    /// Convert absolute pixel coordinates to cell-local normalized (0-1).
    pub fn to_norm(&self, abs_pixels: Vec2) -> Vec2 {
        let local = abs_pixels - self.offset;
        Vec2::new(
            local.x / self.bounds.width,
            local.y / self.bounds.height,
        )
    }

    /// Convert canvas-wide normalized coordinates (0-1) to cell-local normalized (0-1).
    pub fn to_norm_local(&self, norm_abs: Vec2) -> Vec2 {
        let abs_pixels = Vec2::new(
            norm_abs.x * self.work_size.x,
            norm_abs.y * self.work_size.y,
        );
        self.to_norm(abs_pixels)
    }

    /// Get the center point of the cell in cell-local normalized coordinates.
    pub fn center_norm(&self) -> Vec2 {
        Vec2::new(0.5, 0.5)
    }

    /// Get the center point of the cell in canvas-wide normalized coordinates.
    pub fn center_norm_abs(&self) -> Vec2 {
        self.norm_abs(self.center_norm())
    }

    /// Convert cell-local pixel coordinates to absolute pixel coordinates.
    pub fn abs(&self, local_pixels: Vec2) -> Vec2 {
        self.offset + local_pixels
    }

    /// Convert absolute pixel coordinates to cell-local pixel coordinates.
    pub fn to_local(&self, abs_pixels: Vec2) -> Vec2 {
        abs_pixels - self.offset
    }

    /// Get the center point of the cell in absolute pixel coordinates.
    pub fn center(&self) -> Vec2 {
        self.offset
            + Vec2::new(self.bounds.width * 0.5, self.bounds.height * 0.5)
    }

    /// Get the flattened index of this cell (row-major order).
    pub fn index(&self) -> usize {
        (self.row * (self.bounds.width / self.bounds.height) as u32 + self.col) as usize
    }
}
