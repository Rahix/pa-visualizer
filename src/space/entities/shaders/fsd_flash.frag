#version 410

uniform float time;
uniform float start_time;
uniform vec4 bb_position;

smooth in vec2 frag_tex_coord;

out vec4 frg_color;

void main() {
    vec2 pos = frag_tex_coord * 2.0 - 1.0;
    float ang = atan(pos.y / pos.x) + time * 4.0 + bb_position.x;
    float rsq = clamp(sqrt(dot(pos, pos)) + sin(ang * 6.0) * 0.25 + 0.75, 0.0, 1.0);
    float intensity = max(1.0 / ((time - start_time) * 100.0 + 1.0) - 0.1, 0.0);
    float shaped_intensity = (1.0 - pow(rsq, 2.0)) * intensity;
    if(shaped_intensity < 0.01) {
        discard;
    }
    frg_color = vec4(vec3(shaped_intensity*2.0), 1.0*shaped_intensity);
}
