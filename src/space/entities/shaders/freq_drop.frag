#version 410

uniform float time;
uniform float start_time;
uniform float rand;
uniform vec4 bb_position;

smooth in vec2 frag_tex_coord;

out vec4 frg_color;

void main() {
    vec2 pos = frag_tex_coord * 2.0 - 1.0;
    float delta = time - start_time - rand * 0.07;
    float locatione = 1.0 - (pos.y * 0.5 + 0.5) + 1.0 - delta;
    if(locatione > 1.0) {
        discard;
    }
    float intensity = clamp((locatione - 1.0) * 2.0 + 1.0, 0.0, 1.0);

    int colorid = int(floor(rand * 2.0));
    vec3 color = vec3(1.0);
    if(colorid == 0) {
        color = vec3(0.000015, 0.000554, 0.062991) * 1.0;
    } else {
        color = vec3(0.007443, 0.013841, 0.138793) * 1.0;
    }
    frg_color = vec4(color, intensity * 0.5);
}
