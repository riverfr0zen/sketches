#version 450
precision mediump float;
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;

layout(binding = 0) uniform Common {
    float u_time;
    vec2 u_resolution;
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

void main() {
    vec2 st = gl_FragCoord.xy / u_resolution;
    st.x *= u_resolution.x / u_resolution.y;

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
