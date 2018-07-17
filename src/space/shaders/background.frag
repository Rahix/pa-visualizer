#version 410


uniform sampler2D tex_color;
uniform vec2 resolution;
uniform float time;
uniform float beat;

smooth in vec2 frag_texcoord;

out vec4 frg_color;

float PI = 3.1415926;

float radius = 0.15;
float beat_intensity = 0.16;
vec2 position = vec2(-0.2, -0.4);

float circle(vec2 coords) {
    vec2 c = coords + position;
    c.y = c.y * resolution.y / resolution.x;
    float r = sqrt(c.x * c.x + c.y * c.y) * ((1.0 - beat) * beat_intensity + (1.0 - beat_intensity));
    return smoothstep(radius + 0.001, radius - 0.001, r);
}

float stripes(vec2 coords) {
    vec2 c = coords + position;
    c.y = c.y * resolution.y / resolution.x - radius - 0.02;
    float result = 0.0;
    float smoothsize = 0.001 * radius / 0.3;
    float size = 0.008 * radius / 0.3;
    for(int i = 4; i < 16; i++) {
        float y = c.y + pow(i / 2.0, 2) * 0.02 * radius / 0.3;
        result += smoothstep(smoothsize + size, size, y)
                - smoothstep(-size, -size - smoothsize, y);
    }
    return result;
}

vec3 sky(vec2 coords) {
    vec3 result = vec3(0.0);
    float fact = max(circle(coords) - stripes(coords), 0.0);

    // sun
    vec3 color_top = vec3(0.893, 0.346, 0.000246) / 2.0;
    vec3 color_bottom = vec3(0.961, 0.00125, 0.646) / 2.0;
    float color_fact = smoothstep(- radius - position.y, 0.2 * radius / 0.3 + radius - position.y, coords.y);
    vec3 color = color_top * color_fact + (1.0 - color_fact) * color_bottom;

    result += color * fact;

    // gradient
    vec3 color1 = vec3(0.000015, 0.000554, 0.062991) / 15.0;
    vec3 color2 = vec3(0.007443, 0.013841, 0.138793) / 15.0;

    float t = coords.y / 2.0 + 0.5;

    result += (1.0 - fact) * (t * color1 + (1.0 - t) * color2);

    return result;
}

void main() {
    vec4 color = vec4(texture(tex_color, frag_texcoord));
    float alpha = color.a;
    frg_color = vec4(sky(frag_texcoord * 2.0 - 1.0)
            * (1.0 - alpha) + color.rgb * alpha, 1.0);
}

