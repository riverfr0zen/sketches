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

    // Get the four surrounding tile indices
    vec2 tile_floor = floor(tile_coord);
    vec2 tile_frac = fract(tile_coord);

    // Clamp to valid tile indices
    int cols = int(u_grid_size.x);
    int rows = int(u_grid_size.y);

    int x0 = int(tile_floor.x);
    int y0 = int(tile_floor.y);
    int x1 = min(x0 + 1, cols - 1);
    int y1 = min(y0 + 1, rows - 1);

    // Clamp to valid indices
    x0 = clamp(x0, 0, cols - 1);
    y0 = clamp(y0, 0, rows - 1);

    // Get the four corner tile colors
    vec4 c00 = u_tile_colors[y0 * cols + x0]; // top-left
    vec4 c10 = u_tile_colors[y0 * cols + x1]; // top-right
    vec4 c01 = u_tile_colors[y1 * cols + x0]; // bottom-left
    vec4 c11 = u_tile_colors[y1 * cols + x1]; // bottom-right

    // Smoothstep for smoother blending across entire tile
    vec2 smooth_frac = smoothstep(0.8, 1.0, tile_frac);

    // Bilinear interpolation
    vec4 c0 = mix(c00, c10, smooth_frac.x); // top edge
    vec4 c1 = mix(c01, c11, smooth_frac.x); // bottom edge
    vec4 blended = mix(c0, c1, smooth_frac.y);

    color = blended;
}
