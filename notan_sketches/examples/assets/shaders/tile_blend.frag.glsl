#version 450
precision mediump float;
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;

layout(binding = 0) uniform Common {
    float u_time;
    vec2 u_resolution;
};

layout(binding = 1) uniform TileColors {
    vec4 u_tile_colors[225]; // Max 15x15 = 225 tiles
};

layout(binding = 2) uniform TileGridInfo {
    vec2 u_grid_size; // x = cols, y = rows
};

void main() {
    // Use normalized coordinates - Notan's RenderTexture handles Y-flip automatically
    vec2 st = gl_FragCoord.xy / u_resolution;

    // Calculate which tile we're in (fractional coordinates)
    vec2 tile_coord = st * u_grid_size;

    // Get the four surrounding tile indices for bilinear interpolation
    vec2 tile_floor = floor(tile_coord);
    vec2 tile_frac = fract(tile_coord);

    int cols = int(u_grid_size.x);
    int rows = int(u_grid_size.y);

    int x0 = int(tile_floor.x);
    int y0 = int(tile_floor.y);
    int x1 = clamp(x0 + 1, 0, cols - 1);
    int y1 = clamp(y0 + 1, 0, rows - 1);

    x0 = clamp(x0, 0, cols - 1);
    y0 = clamp(y0, 0, rows - 1);

    // Get the four corner tile colors (these are the pure center colors)
    vec4 c00 = u_tile_colors[y0 * cols + x0]; // top-left
    vec4 c10 = u_tile_colors[y0 * cols + x1]; // top-right
    vec4 c01 = u_tile_colors[y1 * cols + x0]; // bottom-left
    vec4 c11 = u_tile_colors[y1 * cols + x1]; // bottom-right

    // Create smooth blend weights that remap tile_frac to blend more near edges
    // This creates a region in the center that stays pure color
    //
    // Blend zone explanation:
    // - blend_start: Distance from center where blending starts (0.3 = 30% from center)
    // - blend_end: Distance from center where blend is complete (0.5 = at edge)
    // - Current values create a blend zone in the outer 20% of each tile (from 30% to 50% distance from center)
    // - Decrease blend_start (e.g., 0.2) for wider blend zones
    // - Increase blend_start (e.g., 0.4) for narrower blend zones
    // - blend_end should stay at 0.5 to blend fully at the tile edge
    float blend_start = 0.3;
    float blend_end = 0.5;

    // Remap tile_frac so blending happens symmetrically from center outward
    vec2 blend_weights;
    blend_weights.x = tile_frac.x < 0.5
        ? smoothstep(blend_start, blend_end, 0.5 - tile_frac.x)
        : smoothstep(blend_start, blend_end, tile_frac.x - 0.5);
    blend_weights.y = tile_frac.y < 0.5
        ? smoothstep(blend_start, blend_end, 0.5 - tile_frac.y)
        : smoothstep(blend_start, blend_end, tile_frac.y - 0.5);

    // Standard bilinear interpolation using the blend weights
    vec4 blend_top = mix(c00, c10, tile_frac.x);
    vec4 blend_bottom = mix(c01, c11, tile_frac.x);
    vec4 bilinear = mix(blend_top, blend_bottom, tile_frac.y);

    // Get the current tile's pure color
    vec4 current_pure = u_tile_colors[y0 * cols + x0];

    // Mix between pure tile color and bilinear blend based on distance from center
    float blend_amount = max(blend_weights.x, blend_weights.y);
    color = mix(current_pure, bilinear, blend_amount);
}
