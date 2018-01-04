#version 410

uniform float time;
uniform float start_time;
uniform float rand;
uniform vec4 bb_position;

smooth in vec2 frag_tex_coord;

out vec4 frg_color;

void main() {
    vec2 pos = frag_tex_coord * 2.0 - 1.0;
    float delta = time - start_time;
    float intensity = max(1.0 / (delta * 20.0 + 1.0) - 0.1, 0.0);
    float shaped_intensity = (exp(-pow(pow(abs(pos.y), 0.1 + delta * 2.0) * 2.0 * (delta * 3.0 + 1.0), 2))
            * exp(-pow(pos.x * 1.5 * (delta * 3.0 + 1.0), 2)))
            * 8.0 * intensity;
    float v = clamp(shaped_intensity, 0.0, 1.0);
    int colorid = int(floor(rand * 4.0));
    vec3 color = vec3(1.0);
    if(colorid == 0) {
        color = vec3(1.0, 1.0, 0.8);
    } else if(colorid == 1) {
        color = vec3(1.0, 0.8, 0.5);
    } else if(colorid == 2) {
        color = vec3(1.0, 1.0, 1.0);
    } else {
        color = vec3(0.8, 0.8, 1.0);
    }
    frg_color = vec4(color * v, v);
}
