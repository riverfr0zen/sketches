use notan::math::{vec2, Vec2};


pub fn mid(vec_a: Vec2, vec_b: Vec2) -> Vec2 {
    vec2((vec_a.x + vec_b.x) / 2.0, (vec_a.y + vec_b.y) / 2.0)
}

/// Deprecated
#[deprecated(note = "Name was too long, use mid() instead")]
pub fn get_vec2_midpoint(vec_a: Vec2, vec_b: Vec2) -> Vec2 {
    mid(vec_a, vec_b)
}


// Returns the sequential position of a cell in a grid where cells are counted
// from left to right and top to bottom.
pub fn get_cell_pos_in_grid(cols_per_row: usize, row: usize, col: usize) -> usize {
    return cols_per_row * row + col;
}
