#version 410


uniform sampler2D tex_color;
uniform vec2 resolution;
uniform float time;
uniform float beat;

smooth in vec2 frag_texcoord;

out vec4 frg_color;

float PI = 3.1415926;

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
    vec3 base_color = vec3(1.0 / 255.0, 0.5 / 255.0, 10.0 / 255.0);
    frg_color = vec4(base_color
            * bg_func3(frag_texcoord * 2.0 - 1.0)
            * (1.0 - alpha) + color.rgb * alpha, 1.0);
}

