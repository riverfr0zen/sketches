#version 450
precision mediump float;
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;


layout(set = 0, binding = 0) uniform Common {
    float u_time;
    float u_resolution_x;
    float u_resolution_y;
};

layout(set = 0, binding = 0) uniform BgColor {
    float bg_color_r;
    float bg_color_g;
    float bg_color_b;
};

layout(set = 0, binding = 0) uniform ColorSource1 {
    float color1_r;
    float color1_g;
    float color1_b;
};

layout(set = 0, binding = 0) uniform ColorSource2 {
    float color2_r;
    float color2_g;
    float color2_b;
};


void main() {
    vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);
    
    float color1_dist = distance(st,vec2(0.2, 0.8));
    // float dist = distance(st,vec2(0.2, abs(sin(u_time))));
    float color1_pct = 0.6-(color1_dist * 1.5);
    // float color1_pct = abs(sin(u_time))-(color1_dist * (1.0 + abs(sin(u_time))));
    if (color1_pct < 0.0) {
        color1_pct = 0.0;
    }
    vec3 color1 = vec3(color1_r, color1_g, color1_b);


    float color2_dist = distance(st,vec2(0.5, 0.5));
    // float dist = distance(st,vec2(0.2, abs(sin(u_time))));
    float color2_pct = 0.6-(color2_dist * 1.5);
    // float color2_pct = abs(sin(u_time))-(color2_dist * (1.0 + abs(sin(u_time))));
    if (color2_pct < 0.0) {
        color2_pct = 0.0;
    }
    vec3 color2 = vec3(color2_r, color2_g, color2_b);

    vec3 bg_color = vec3(bg_color_r, bg_color_g, bg_color_b);

    vec3 xcolor = mix(bg_color, color1, color1_pct);
    // vec3 xcolor = mix(bg_color, color1, color1_pct * abs(sin(u_time)));

    xcolor = mix(xcolor, color2, color2_pct);


    color = vec4(xcolor, 1.0);
}
