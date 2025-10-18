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
};

float random (in float x) {
    return fract(sin(x)*1e4);
}

float random (in vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898,78.233)))* 43758.5453123);
}

float pattern(vec2 st, vec2 v, float t) {
    vec2 p = floor(st+v);
    return step(t, random(cos(u_time/1e6)+p*0.00000001)+random(p.x)*0.5 );
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
    // The curve offset is relative to strip_y, so we need to adjust st.y accordingly
    float strip_y_norm = strip_y + curve_offset;
    float pixel_y_norm = st.y;

    // Shift the pattern coordinates to follow the curve
    st.y = st.y - curve_offset;

    vec2 grid = vec2(100.0, 50.0);
    st *= grid;

    vec2 ipos = floor(st);  // integer
    vec2 fpos = fract(st);  // fraction

    vec2 vel = vec2(u_time * 0.5 * max(grid.x, grid.y)); // time
    vel *= vec2(-1.0, 0.0) * random(1.0 + ipos.y); // direction

    // Assign a random value base on the integer coord
    vec2 offset = vec2(cos(u_time * 1e5));

    vec3 out_color = vec3(0.905, 0.613, 0.081);
    out_color.r = pattern(st + offset, vel, max(random(abs(tan(u_time))), 0.5));
    out_color.g = pattern(st, vel, 0.5);
    out_color.b = pattern(st - offset, vel, 0.5);

    // Margins
    out_color *= step(0.2, fpos.y);
    out_color *= step(abs(sin(u_time)), fpos.y) - step(u_time, fpos.x / fpos.y);

    color = vec4(out_color * (0.8 + step(random(ipos.y), random(vel.y))), 1.0);
}
