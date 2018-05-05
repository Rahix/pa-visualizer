#version 410


uniform sampler2D tex_color;
uniform vec2 resolution;
uniform float time;
uniform float beat;

smooth in vec2 frag_texcoord;

out vec4 frg_color;

float PI = 3.1415926;

float radius = 0.3;
vec2 position = vec2(-0.3, 0.2);

float circle(vec2 coords) {
    vec2 c = coords + position;
    c.y = c.y * resolution.y / resolution.x;
    float r = sqrt(c.x * c.x + c.y * c.y) * ((1.0 - beat) * 0.1 + 0.9);
    return smoothstep(radius + 0.01, radius - 0.01, r);
}

float stripes(vec2 coords) {
    vec2 c = coords + position;
    c.y = c.y * resolution.y / resolution.x - 0.06;
    return 0.0
        + smoothstep(0.02, 0.01, c.y) - smoothstep(-0.01, -0.02, c.y)
        + smoothstep(0.02, 0.01, c.y + 0.1) - smoothstep(-0.01, -0.02, c.y + 0.1)
        + smoothstep(0.02, 0.01, c.y + 0.23) - smoothstep(-0.01, -0.02, c.y + 0.23)
        + smoothstep(0.02, 0.01, c.y - 0.08) - smoothstep(-0.01, -0.02, c.y - 0.08)
        + smoothstep(0.02, 0.01, c.y - 0.15) - smoothstep(-0.01, -0.02, c.y - 0.15)
    ;
}

vec3 sun(vec2 coords) {
    float fact = max(circle(coords) - stripes(coords), 0.0);
    vec3 color_top = vec3(0.937, 0.502, 0.059);
    vec3 color_bottom = vec3(0.878, 0.275, 0.824) * 0.4;
    float color_fact = smoothstep(-0.8, 1.0, coords.y);
    vec3 color = color_top * color_fact + (1.0 - color_fact) * color_bottom;
    return color * fact * 0.5;
}

void main() {
    vec4 color = vec4(texture(tex_color, frag_texcoord));
    float alpha = color.a;
    vec3 base_color = vec3(0.14 / 7.0, 0.10 / 7.0, 0.16 / 7.0);
    frg_color = vec4(sun(frag_texcoord * 2.0 - 1.0)
            * (1.0 - alpha) + color.rgb * alpha, 1.0);
}

