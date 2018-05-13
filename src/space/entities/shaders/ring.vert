#version 410

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;
uniform vec4 color;

in vec4 position;
in float idx;

smooth out vec4 frag_color;
smooth out float theta;

void main() {
    frag_color = color;
    theta = idx;
    vec4 world_position = model_matrix * position;
    gl_Position = perspective_matrix * view_matrix * world_position;
}
