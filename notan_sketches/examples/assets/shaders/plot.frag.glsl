#version 450
precision mediump float;
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;

layout(set = 0, binding = 2) uniform Common {
    float u_time;
    float u_resolution_x;
    float u_resolution_y;
};

// Plot a line on Y using a value between 0.0-1.0
float plot(vec2 st) {
    return smoothstep(0.02, 0.0, abs(st.y - st.x));
}

float plot2(vec2 st, float pct){
    return  smoothstep( pct-0.02, pct, st.y) -
            smoothstep( pct, pct+0.02, st.y);
}

void main() {
    vec2 st = gl_FragCoord.xy / vec2(u_resolution_x, u_resolution_y);

    // float y = st.y;
    float y = pow(st.x,5.0);

    vec3 xcolor = vec3(y);

    // Plot a line
    // float pct = plot(st);
    float pct = plot2(st, y);

    xcolor = (1.0-pct)*xcolor+pct*vec3(0.0,1.0,0.0);
    // color = vec4(xcolor, 1.0);
    color = vec4(xcolor, 1.0 * abs(sin(u_time)));
}
