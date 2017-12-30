#version 410

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;
uniform vec4 color;

in vec4 position;

smooth out vec4 frag_color;

void main() {
    frag_color = color;
    vec4 world_position = model_matrix * position;
    gl_Position = perspective_matrix * view_matrix * world_position;
}
