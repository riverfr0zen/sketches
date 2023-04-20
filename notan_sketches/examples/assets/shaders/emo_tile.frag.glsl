#version 450
precision mediump float;
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;


layout(set = 0, binding = 0) uniform Common {
    float u_time;
    float u_resolution_x;
    float u_resolution_y;
};

// layout(set = 0, binding = 0) uniform TileColors {
//     vec3 tile_color;
//     vec3 bg_color;
// };

layout(set = 0, binding = 0) uniform TileColors {
    float tile_color_r;
    float tile_color_g;
    float tile_color_b;
    float bg_color_r;
    float bg_color_g;
    float bg_color_b;
};


void main() {
    vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
    
    // float pct = 0.5-distance(st,vec2(0.5));
    // float pct = 1.0-distance(st,vec2(0.5));
    // float pct = 0.8-distance(st,vec2(0.5));
    float dist = distance(st,vec2(0.5));
    float pct = 0.6-(dist * 1.2);
    if (pct < 0.0) {
        pct = 0.0;
    }

    vec3 tile_color = vec3(tile_color_r, tile_color_g, tile_color_b);
    vec3 bg_color = vec3(bg_color_r, bg_color_g, bg_color_b);

    vec3 xcolor = mix(bg_color, tile_color, pct);
    // vec3 xcolor = mix(bg_color, tile_color, pct * abs(sin(u_time)));

    // color = vec4(xcolor, 1.0);
    color = vec4(xcolor, 0.7);
}
