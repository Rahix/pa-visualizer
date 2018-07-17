#version 410

uniform sampler2D tex_color;
uniform vec2 resolution;
uniform bool horizontal;

smooth in vec2 frag_texcoord;

out vec4 frg_color;

#define GAUSS_SAMPLES 16
float weights[GAUSS_SAMPLES] = float[] ( 0.220768, 0.149118, 0.081492, 0.054233, 0.039035, 0.027081, 0.017436, 0.010366, 0.005689, 0.002882, 0.001348, 0.000582, 0.000232, 0.000085, 0.000029, 0.000009);

vec4 gauss(sampler2D tex, vec2 coords) {
    vec2 tex_offset = 1.0 / textureSize(tex, 0);
    vec4 result = texture(tex, coords) * weights[0];

    if(horizontal) {
        tex_offset.y = 0.0;
    } else {
        tex_offset.x = 0.0;
    }

    for(int i = 1; i < GAUSS_SAMPLES; i++) {
        result += texture(tex, coords + tex_offset * i) * weights[i];
        result += texture(tex, coords - tex_offset * i) * weights[i];
    }

    return result;
}

void main() {
    frg_color = gauss(tex_color, frag_texcoord);
}
