#version 410

smooth in vec4 frag_position;
smooth in vec4 frag_screen_position;
smooth in vec4 frag_color;

out vec4 fg_position;
out vec4 fg_screen_position;
out vec4 fg_color;

void main() {
    fg_position = vec4(frag_position);
    fg_screen_position = vec4(frag_screen_position);
    fg_color = vec4(frag_color);
}
