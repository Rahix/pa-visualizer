#version 410

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform vec4 bb_position;
uniform vec2 size;

in vec2 position;
in vec2 tex_coord;

smooth out vec2 frag_tex_coord;

void main() {
    frag_tex_coord = tex_coord;
    gl_Position = perspective_matrix * view_matrix * bb_position
        + perspective_matrix * vec4(position * (size / 2.0), 0.0, 1.0);
}
