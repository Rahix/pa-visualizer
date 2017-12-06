#version 410

in vec4 position;
in vec2 texcoord;

smooth out vec2 frag_texcoord;

void main() {
    gl_Position = position;
    frag_texcoord = texcoord;
}
