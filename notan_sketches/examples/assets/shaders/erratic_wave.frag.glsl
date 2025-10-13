#version 450
precision mediump float;
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;

layout(binding = 0) uniform Common {
    float u_time;
    vec2 u_resolution;
};

float plot(vec2 st, float pct, float feather) {
    return smoothstep(pct + feather, pct, st.y);
}


float plot2(vec2 st, float pct, float top_feather, float bottom_feather) {
    return smoothstep(pct - bottom_feather, pct, st.y) - smoothstep(pct, pct + top_feather, st.y);
}


// Divide by 2.0 to scale down y coordinates since display coord system does not have "negative coordinates".
// After scaling down, compensate for half the wave being in negative y coords by adding 0.5
// and thus pushing the full sine wave upwards
float adjusted_sin(float x, float y_shrink, float wave_height) {
    // return sin(x) / 2.0 + wave_height;
    // return sin(x) / 20.0 + wave_height;
    return sin(x) / y_shrink + wave_height;
}


void main() {
    vec2 st = gl_FragCoord.xy / u_resolution;
    vec3 backgroundColor = vec3(1.0, 0.65, 0.2) * ((1-st.y) * 4.0);
    vec3 waveColor = vec3(0.043, 0.525, 0.756) * (st.y * 0.8);

    float wave_height = 0.5;
    float max_y_shrink = 30.0;
    float min_y_shrink = 10.0;
    float wave_y_timeframe = mod(u_time, max_y_shrink);
    float wave_y_timeframe2x = mod(u_time, (max_y_shrink * 2.0));
    float wave_y_shrink = wave_y_timeframe;

    // Using the 2x timeframe to step `wave_y_shrink` "backwards" if we've gone past the
    // single-direction timeframe. This is a technique that can be used to get a 
    // "pendulum" or "back-and-forth" effect from time and modulus.
    if (wave_y_timeframe2x > max_y_shrink) {
        wave_y_shrink = max_y_shrink - wave_y_timeframe;
    }
    if (wave_y_shrink < min_y_shrink) {
        wave_y_shrink = min_y_shrink;
    }

    float y = adjusted_sin(st.x * abs(sin(mod(u_time, 60.0))) * 5.5 + u_time, wave_y_shrink, wave_height);

    // float pct = plot2(st, y, 0.02, 0.02);
    float pct = plot2(st, y, 0.05, 50.0);
    // float pct = plot(st, y, 0.05);

    waveColor = mix(backgroundColor, waveColor, pct);
    color = vec4(waveColor, 1.0);
}
