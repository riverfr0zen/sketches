use notan::math::vec2;
use notan::prelude::Random;
use notan_sketches::gridutils::*;

// Helper function for common test setup
fn create_test_grid() -> Grid<i32> {
    Grid::builder(3, 3, vec2(300.0, 300.0))
        .with_cell_data(|row, col, _bounds, _rng| (row * 3 + col) as i32)
        .build(&mut Random::default())
}

// ===== Core Grid Functionality =====

#[test]
fn test_grid_creation_dimensions() {
    let grid = create_test_grid();
    assert_eq!(grid.rows(), 3);
    assert_eq!(grid.cols(), 3);
    assert_eq!(grid.total_cells(), 9);
}

#[test]
fn test_cell_dimensions_calculated_correctly() {
    let grid = create_test_grid();
    assert_eq!(grid.cell_width(), 100.0);
    assert_eq!(grid.cell_height(), 100.0);
}

#[test]
fn test_grid_with_different_aspect_ratios() {
    let grid = Grid::builder(2, 4, vec2(800.0, 400.0))
        .with_cell_data(|row, col, _bounds, _rng| (row, col))
        .build(&mut Random::default());

    assert_eq!(grid.cell_width(), 200.0);
    assert_eq!(grid.cell_height(), 200.0);
}

#[test]
fn test_single_cell_grid() {
    let grid = Grid::builder(1, 1, vec2(100.0, 100.0))
        .with_cell_data(|_, _, _bounds, _rng| 42)
        .build(&mut Random::default());

    assert_eq!(grid.total_cells(), 1);
    assert_eq!(grid.cell_width(), 100.0);
}

// ===== Cell Indexing =====

#[test]
fn test_cell_iteration_visits_all_cells_once() {
    let grid = create_test_grid();
    let count = grid.cells().count();
    assert_eq!(count, 9);
}

#[test]
fn test_cell_iteration_order_is_row_major() {
    let grid = create_test_grid();
    let data: Vec<i32> = grid.cells().map(|cell| *cell.data).collect();
    assert_eq!(data, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_get_returns_correct_cell() {
    let grid = create_test_grid();
    let cell = grid.get(1, 1).unwrap();
    assert_eq!(*cell.data, 4); // Middle cell: row 1 * 3 + col 1 = 4
    assert_eq!(cell.row, 1);
    assert_eq!(cell.col, 1);
}

#[test]
fn test_get_out_of_bounds_returns_none() {
    let grid = create_test_grid();
    assert!(grid.get(3, 0).is_none());
    assert!(grid.get(0, 3).is_none());
    assert!(grid.get(10, 10).is_none());
}

// ===== Coordinate Transformations - Normalized (CRITICAL!) =====

#[test]
fn test_norm_at_origin() {
    let grid = create_test_grid();
    let cell = grid.get(0, 0).unwrap();

    let abs_pos = cell.norm(vec2(0.0, 0.0));

    // First cell offset is (0, 0), normalized (0, 0) = absolute (0, 0)
    assert_eq!(abs_pos, vec2(0.0, 0.0));
}

#[test]
fn test_norm_at_center() {
    let grid = create_test_grid();
    let cell = grid.get(1, 1).unwrap(); // Middle cell

    let abs_pos = cell.norm(vec2(0.5, 0.5));

    // Middle cell offset is (100, 100), center is at +50,+50
    assert_eq!(abs_pos, vec2(150.0, 150.0));
}

#[test]
fn test_norm_at_max() {
    let grid = create_test_grid();
    let cell = grid.get(0, 0).unwrap();

    let abs_pos = cell.norm(vec2(1.0, 1.0));

    // First cell, normalized (1, 1) = bottom-right of cell = (100, 100)
    assert_eq!(abs_pos, vec2(100.0, 100.0));
}

#[test]
fn test_norm_abs_converts_to_canvas_normalized() {
    let grid = create_test_grid();
    let cell = grid.get(1, 1).unwrap(); // Middle cell

    let canvas_norm = cell.norm_abs(vec2(0.5, 0.5));

    // Middle cell center at (150, 150), canvas is 300x300
    // So canvas norm = (150/300, 150/300) = (0.5, 0.5)
    assert_eq!(canvas_norm, vec2(0.5, 0.5));
}

#[test]
fn test_to_norm_reverse_conversion() {
    let grid = create_test_grid();
    let cell = grid.get(1, 1).unwrap();

    let abs_pos = vec2(125.0, 175.0);
    let norm_pos = cell.to_norm(abs_pos);

    // Cell offset is (100, 100), so local = (25, 75)
    // Norm = (25/100, 75/100) = (0.25, 0.75)
    assert_eq!(norm_pos, vec2(0.25, 0.75));
}

#[test]
fn test_center_norm_always_returns_half() {
    let grid = create_test_grid();

    for cell in grid.cells() {
        assert_eq!(cell.center_norm(), vec2(0.5, 0.5));
    }
}

#[test]
fn test_center_norm_abs_varies_by_cell() {
    let grid = create_test_grid();

    // Top-left cell center
    let cell = grid.get(0, 0).unwrap();
    let center = cell.center_norm_abs();
    assert_eq!(center, vec2(50.0 / 300.0, 50.0 / 300.0));

    // Middle cell center
    let cell = grid.get(1, 1).unwrap();
    let center = cell.center_norm_abs();
    assert_eq!(center, vec2(150.0 / 300.0, 150.0 / 300.0));
}

// ===== Coordinate Transformations - Pixels =====

#[test]
fn test_abs_converts_local_pixels_to_absolute() {
    let grid = create_test_grid();
    let cell = grid.get(1, 2).unwrap(); // Row 1, Col 2

    let abs_pos = cell.abs(vec2(25.0, 50.0));

    // Cell offset is (200, 100), so absolute = (225, 150)
    assert_eq!(abs_pos, vec2(225.0, 150.0));
}

#[test]
fn test_to_local_reverse_conversion() {
    let grid = create_test_grid();
    let cell = grid.get(1, 2).unwrap();

    let abs_pos = vec2(225.0, 150.0);
    let local_pos = cell.to_local(abs_pos);

    // Cell offset is (200, 100), so local = (25, 50)
    assert_eq!(local_pos, vec2(25.0, 50.0));
}

#[test]
fn test_center_returns_absolute_pixel_center() {
    let grid = create_test_grid();
    let cell = grid.get(1, 1).unwrap();

    let center = cell.center();

    // Middle cell offset (100, 100), cell size (100, 100)
    // Center = (150, 150)
    assert_eq!(center, vec2(150.0, 150.0));
}

// ===== Canvas-Wide Helpers =====

#[test]
fn test_grid_norm_to_pixels_full_canvas() {
    let grid = create_test_grid();

    assert_eq!(grid.norm_to_pixels(vec2(0.0, 0.0)), vec2(0.0, 0.0));
    assert_eq!(grid.norm_to_pixels(vec2(0.5, 0.5)), vec2(150.0, 150.0));
    assert_eq!(grid.norm_to_pixels(vec2(1.0, 1.0)), vec2(300.0, 300.0));
}

#[test]
fn test_grid_pixels_to_norm_full_canvas() {
    let grid = create_test_grid();

    assert_eq!(grid.pixels_to_norm(vec2(0.0, 0.0)), vec2(0.0, 0.0));
    assert_eq!(grid.pixels_to_norm(vec2(150.0, 150.0)), vec2(0.5, 0.5));
    assert_eq!(grid.pixels_to_norm(vec2(300.0, 300.0)), vec2(1.0, 1.0));
}

#[test]
fn test_roundtrip_canvas_conversions() {
    let grid = create_test_grid();
    let original = vec2(123.456, 234.567);

    let norm = grid.pixels_to_norm(original);
    let back = grid.norm_to_pixels(norm);

    assert!((back.x - original.x).abs() < 0.001);
    assert!((back.y - original.y).abs() < 0.001);
}

// ===== Iteration =====

#[test]
fn test_cells_iterator_is_immutable() {
    let grid = create_test_grid();

    for cell in grid.cells() {
        // cell.data is &i32, not &mut i32
        let _ = *cell.data;
    }
}

#[test]
fn test_cells_mut_allows_mutation() {
    let mut grid = create_test_grid();

    for mut cell in grid.cells_mut() {
        *cell.data += 100;
    }

    // Verify mutations persisted
    let data: Vec<i32> = grid.cells().map(|cell| *cell.data).collect();
    assert_eq!(data, vec![100, 101, 102, 103, 104, 105, 106, 107, 108]);
}

#[test]
fn test_iteration_provides_correct_row_col() {
    let grid = create_test_grid();

    let cell = grid.get(2, 1).unwrap();
    assert_eq!(cell.row, 2);
    assert_eq!(cell.col, 1);
}

#[test]
fn test_iteration_provides_correct_bounds() {
    let grid = create_test_grid();

    for cell in grid.cells() {
        assert_eq!(cell.bounds.width, 100.0);
        assert_eq!(cell.bounds.height, 100.0);
    }
}

#[test]
fn test_iteration_provides_correct_offset() {
    let grid = create_test_grid();

    let cell = grid.get(1, 2).unwrap();
    assert_eq!(cell.offset, vec2(200.0, 100.0));
}

// ===== Data Management =====

#[test]
fn test_regenerate_cells_replaces_all_data() {
    let mut grid = create_test_grid();

    grid.regenerate_cells(&mut Random::default(), |row, col, _bounds, _rng| {
        (row * 10 + col) as i32
    });

    let data: Vec<i32> = grid.cells().map(|cell| *cell.data).collect();
    assert_eq!(data, vec![0, 1, 2, 10, 11, 12, 20, 21, 22]);
}

#[test]
fn test_get_mut_allows_modification() {
    let mut grid = create_test_grid();

    if let Some(mut cell) = grid.get_mut(1, 1) {
        *cell.data = 999;
    }

    assert_eq!(*grid.get(1, 1).unwrap().data, 999);
}

#[test]
fn test_cell_data_persists_across_iterations() {
    let mut grid = create_test_grid();

    // Modify in first iteration
    for mut cell in grid.cells_mut() {
        *cell.data *= 2;
    }

    // Verify in second iteration
    for cell in grid.cells() {
        assert_eq!(*cell.data % 2, 0);
    }
}

// ===== Edge Cases =====

#[test]
fn test_non_square_cells() {
    let grid = Grid::builder(2, 3, vec2(600.0, 400.0))
        .with_cell_data(|_, _, _bounds, _rng| 0)
        .build(&mut Random::default());

    assert_eq!(grid.cell_width(), 200.0);
    assert_eq!(grid.cell_height(), 200.0);
}

#[test]
fn test_fractional_cell_dimensions() {
    let grid = Grid::builder(3, 3, vec2(100.0, 100.0))
        .with_cell_data(|_, _, _bounds, _rng| 0)
        .build(&mut Random::default());

    // 100 / 3 = 33.333...
    assert!((grid.cell_width() - 33.333333).abs() < 0.001);
}
