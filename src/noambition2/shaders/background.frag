#version 410

uniform sampler2D tex_position;
uniform sampler2D tex_screen_position;
uniform sampler2D tex_color;
uniform float time;
uniform float beat;
uniform vec2 resolution;

smooth in vec2 frag_texcoord;

out vec4 fg_position;
out vec4 fg_screen_position;
out vec4 fg_color;

float PI = 3.1415926;

// Debug
float bg_func1(vec2 coords) {
    vec2 c = coords;
    c.y = c.y * resolution.y / resolution.x;
    float factor = 5.0 * (1.0 - beat);
    c *= (1.0 - beat) * 0.8 + 3.0;
    return 1.0 - smoothstep(-factor, factor, pow(c.x, 2.0) + pow(c.y, 2.0) - 1.0) * 0.9;
}

// .-.. . -. .-
float bg_func2(vec2 coords) {
    vec2 c = coords;
    c.y = c.y * resolution.y / resolution.x;
    c.y += 0.1;
    float factor = 5.0 * (1.0 - beat);
    c *= (1.0 - beat) * 0.8 + 3.0;
    return 1.0 - smoothstep(-factor, factor, pow(c.x, 2.0) + pow(c.y - pow(pow(c.x, 2.0), 1.0 / 3.0), 2.0) - 1.0) * 0.8;
}

// Production
float bg_func3(vec2 coords) {
    vec2 c = coords;
    c.y = c.y * resolution.y / resolution.x;
    float exponent = 3.0 * (1.0 - beat) + 7.0;
    return max(pow(cos(PI * c.x / 2.0), exponent)
             * pow(cos(PI * c.y / 2.0), exponent) * 0.9 + 0.1, 0.1);
}

void main() {
    vec4 color = vec4(texture(tex_color, frag_texcoord));
    float alpha = color.a;
    vec3 base_color = vec3(7.0 / 255.0, 5.0 / 255.0, 40.0 / 255.0);
    fg_color = vec4(base_color
            * bg_func3(frag_texcoord * 2.0 - 1.0)
            * (1.0 - alpha) + color.rgb * alpha, 1.0);
    fg_position = vec4(texture(tex_position, frag_texcoord));
    fg_screen_position = vec4(texture(tex_screen_position, frag_texcoord));
}

