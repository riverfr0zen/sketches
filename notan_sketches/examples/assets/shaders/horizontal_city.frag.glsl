#version 450
precision mediump float;
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;

layout(binding = 0) uniform Common {
    float u_time;
    vec2 u_resolution;
};

layout(binding = 1) uniform CurveData {
    vec4 s0, s1, s2, s3, s4, s5, s6, s7;  // 8 Vec4s = 32 samples
    float strip_y;          // Base Y position of the strip
    float strip_height;     // Height of the strip
    float num_samples;      // Number of valid samples
    float _padding;         // Alignment padding
    vec4 bg_color;          // Background color of the strip (rgba)
};

float random (in float x) {
    return fract(sin(x)*1e4);
}

float random (in vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898,78.233)))* 43758.5453123);
}

// Returns vec2: x = circle shape, y = random opacity for this cell
vec2 pattern(vec2 st, vec2 v, float t) {
    vec2 p = floor(st+v);
    vec2 f = fract(st+v);

    // Create circular patterns instead of squares
    // Center the coordinate at (0.5, 0.5) within each cell
    vec2 centered = f - 0.5;

    // Calculate distance from center for perfectly circular shape
    float dist = length(centered);

    // Use distance to create circular patterns with fixed sizes
    float radius = 0.3 + random(p.y) * 0.2; // Random radius between 0.3 and 0.5

    // Simple threshold without time-based variation
    float rand_val = random(p);
    float circle = step(dist, radius) * step(t, rand_val);

    // Random opacity per cell (between 0.3 and 1.0 for visibility)
    float opacity = 0.3 + random(p * 1.5) * 0.7;

    return vec2(circle, opacity);
}

// Get a single sample value from the packed vec4s
float get_sample(int idx) {
    int vec_idx = idx / 4;
    int comp_idx = idx % 4;

    vec4 v;
    if (vec_idx == 0) v = s0;
    else if (vec_idx == 1) v = s1;
    else if (vec_idx == 2) v = s2;
    else if (vec_idx == 3) v = s3;
    else if (vec_idx == 4) v = s4;
    else if (vec_idx == 5) v = s5;
    else if (vec_idx == 6) v = s6;
    else v = s7;

    if (comp_idx == 0) return v.x;
    else if (comp_idx == 1) return v.y;
    else if (comp_idx == 2) return v.z;
    else return v.w;
}

// Get the curve offset at a given normalized x position
float get_curve_offset(float x_norm) {
    // Sample the curve
    float sample_idx = x_norm * (num_samples - 1.0);
    int idx0 = int(floor(sample_idx));
    int idx1 = min(idx0 + 1, int(num_samples) - 1);

    // Clamp indices
    idx0 = clamp(idx0, 0, int(num_samples) - 1);
    idx1 = clamp(idx1, 0, int(num_samples) - 1);

    // Interpolate between samples
    float frac = fract(sample_idx);
    return mix(get_sample(idx0), get_sample(idx1), frac);
}

void main() {
    vec2 st = gl_FragCoord.xy / u_resolution;
    st.x *= u_resolution.x / u_resolution.y;

    // Get the curve offset for this x position
    float x_norm = gl_FragCoord.x / u_resolution.x;
    float curve_offset = get_curve_offset(x_norm);

    // Warp the y coordinate based on the curve
    // When the curve goes up (positive offset), shift the pattern up to follow
    st.y = st.y + curve_offset;

    vec2 grid = vec2(100.0, 50.0);
    st *= grid;

    vec2 ipos = floor(st);  // integer
    vec2 fpos = fract(st);  // fraction

    vec2 vel = vec2(u_time * 0.5 * max(grid.x, grid.y)); // time
    vel *= vec2(-1.0, 0.0) * random(1.0 + ipos.y); // direction

    // Assign a random value base on the integer coord
    vec2 offset = vec2(0.1); // Fixed offset instead of time-based

    vec3 pattern_color = vec3(0.905, 0.613, 0.081);

    // Get pattern shape and opacity for each color channel
    vec2 pattern_r = pattern(st + offset, vel, 0.5); // Fixed threshold
    vec2 pattern_g = pattern(st, vel, 0.5);
    vec2 pattern_b = pattern(st - offset, vel, 0.5);

    pattern_color.r = pattern_r.x;
    pattern_color.g = pattern_g.x;
    pattern_color.b = pattern_b.x;

    // Use the average opacity from the patterns that are visible
    float avg_opacity = (pattern_r.y * pattern_r.x + pattern_g.y * pattern_g.x + pattern_b.y * pattern_b.x) / max(pattern_r.x + pattern_g.x + pattern_b.x, 1.0);

    // Calculate pattern presence (1.0 if any pattern visible, 0.0 if not)
    float has_pattern = min(pattern_r.x + pattern_g.x + pattern_b.x, 1.0);

    // Apply the pattern's random opacity to blend pattern color with strip background
    vec3 pattern_with_brightness = pattern_color * (0.8 + step(random(ipos.y), random(vel.y)));

    // Blend pattern with background based on both presence and per-ellipse opacity
    // When has_pattern=1 and avg_opacity is low, show more of the strip background
    vec3 out_color = mix(bg_color.rgb, pattern_with_brightness, has_pattern * avg_opacity);

    // Keep the strip itself fully opaque (alpha=1.0)
    color = vec4(out_color, 1.0);
}
