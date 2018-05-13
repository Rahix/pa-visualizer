#version 410

uniform float volume;
uniform float time;

smooth in vec4 frag_color;
smooth in float theta;

out vec4 frg_color;

float d = 0.005;

void main() {
    float fact = smoothstep(0.5 - d, 0.5 + d, theta + volume * 0.1 - 0.05);
    vec3 base = frag_color.xyz;
    vec3 color = base * fact + (1.0 - fact) * (vec3(1.0) - base) * 0.5;
    frg_color = vec4(color * 0.7, fact);
}
