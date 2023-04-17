#version 450
precision mediump float;
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;


layout(set = 0, binding = 0) uniform Common {
    float u_time;
    vec2 u_resolution;
};

layout(set = 0, binding = 0) uniform BgColor {
    vec3 bg_color;
};

layout(set = 0, binding = 0) uniform ColorSource1 {
    vec3 color1;
    vec2 color1_pos;
};

layout(set = 0, binding = 0) uniform ColorSource2 {
    vec3 color2;
    vec2 color2_pos;
};


float get_distance(in vec2 point1, in vec2 point2) {
    return distance(point1, point2);
    // return distance(point1, vec2(point2.x, abs(sin(u_time))));
}


float get_pct(in float distance) {
    float pct = 0.6-(distance * 1.5);
    // float pct = abs(sin(u_time))-(distance * (1.0 + abs(sin(u_time))));
    if (pct < 0.0) {
        return 0.0;
    }
    return pct;
}

void main() {
    vec2 st = gl_FragCoord.xy / u_resolution;
    
    float color1_dist = get_distance(st,vec2(0.2, 0.8));
    float color1_pct = get_pct(color1_dist);

    float color2_dist = get_distance(st,vec2(0.5, 0.5));
    float color2_pct = get_pct(color2_dist);

    vec3 xcolor = mix(bg_color, color1, color1_pct);
    // vec3 xcolor = mix(bg_color, color1, color1_pct * abs(sin(u_time)));

    xcolor = mix(xcolor, color2, color2_pct);


    color = vec4(xcolor, 1.0);
}
