#version 410

smooth in vec4 frag_color;

out vec4 frg_color;

void main() {
    frg_color = frag_color;
}
