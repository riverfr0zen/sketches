use notan::math::{vec2, Vec2};


pub fn get_vec2_midpoint(vec_a: Vec2, vec_b: Vec2) -> Vec2 {
    vec2((vec_a.x + vec_b.x) / 2.0, (vec_a.y + vec_b.y) / 2.0)
}
