#version 410

uniform sampler2D tex_color;
uniform sampler2D tex_gaussed;
uniform vec2 resolution;

smooth in vec2 frag_texcoord;

out vec4 frg_color;

void main() {
    vec4 color = texture(tex_color, frag_texcoord);
    vec4 bloom = texture(tex_gaussed, frag_texcoord);
    frg_color = (color + bloom * 2.0) / 2.0;
}
