#version 410

uniform sampler2D tex_position;
uniform sampler2D tex_screen_position;
uniform sampler2D tex_color;

smooth in vec2 frag_texcoord;

out vec4 frag_output;

void main() {
    frag_output = vec4(texture(tex_color, frag_texcoord).rgb, 1.0);
    //frag_output = vec4(frag_texcoord, 1.0, 1.0);
}
