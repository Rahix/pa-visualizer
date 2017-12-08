#version 410

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

in vec4 position;
in vec4 color;

smooth out vec4 frag_position;
smooth out vec4 frag_screen_position;
smooth out vec4 frag_color;

void main() {
    frag_position = model_matrix * position;
    //frag_position.z += sin(frag_position.y ) * pow(frag_position.y / 30.0, 3.0) * 3.0 * (pow(frag_position.x / 10.0, 2.0) * 2.0 + 0.5);
    frag_position.z += exp(-pow(frag_position.y / 5.0 - 4.0, 2.0)) * (pow(frag_position.x / 10.0, 2.0) * 2.0 + 0.5);
    frag_color = color;
    frag_color.a = frag_color.a * (1.0 - smoothstep(15.0, 25.0, frag_position.y));
    //frag_color.a = frag_color.a * (1.0 - smoothstep(25.0, 30.0, frag_position.y));
    frag_screen_position = perspective_matrix * view_matrix * frag_position;
    gl_Position = frag_screen_position;
}
