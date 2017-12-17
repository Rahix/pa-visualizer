#version 410

uniform sampler2D tex_position;
uniform sampler2D tex_screen_position;
uniform sampler2D tex_color;
uniform float time;
uniform float beat;
uniform float volume;
uniform vec2 resolution;

smooth in vec2 frag_texcoord;

out vec4 fg_position;
out vec4 fg_screen_position;
out vec4 fg_color;

void main() {
    float speed = 1.0;
    mat3 color_matrix = mat3(
            vec3(sin(time*speed)*0.5+0.5, 0.0, -sin(time*speed)*0.5+0.5),
            vec3(0.0, 1.0, 0.0),
            vec3(-sin(time*speed)*0.5+0.5, 0.0, sin(time*speed)*0.5+0.5)
    );
    /*mat3 color_matrix2 = mat3(
            vec3(sin(time*speed+time_offset)*0.5+0.5, 0.0, -sin(time*speed+time_offset)*0.5+0.5),
            vec3(0.0, 1.0, 0.0),
            vec3(-sin(time*speed+time_offset)*0.5+0.5, 0.0, sin(time*speed+time_offset)*0.5+0.5)
    );
    mat3 color_matrix = mat3(
            vec3(sin(time*speed)*0.5+0.5, 0.0, 0.0),
            vec3(0.0, sin(time*speed)*0.5+0.5, 0.0),
            vec3(-sin(time*speed)*0.5+0.5*0.2, 0.0, 1.0)
    );*/
    float t_offset = 3.1415926;
    float s_offset = 0.8;
    mat3 color_matrix2 = mat3(
            vec3(1.0 - (sin((time+t_offset)*(speed+s_offset))*0.5+0.5)*0.4,
                    (sin((time+t_offset)*(speed+s_offset))*0.5+0.5)*0.4+0.2,
                    (sin((time+t_offset)*(speed+s_offset))*0.5+0.5)*0.4+0.2),
            vec3((sin((time+t_offset)*(speed+s_offset))*0.5+0.5)*0.4+0.2,
                    1.0 - (sin((time+t_offset)*(speed+s_offset))*0.5+0.5)*0.4,
                    (sin((time+t_offset)*(speed+s_offset))*0.5+0.5)*0.4+0.2),
            vec3((sin((time+t_offset)*(speed+s_offset))*0.5+0.5)*0.4+0.2,
                    (sin((time+t_offset)*(speed+s_offset))*0.5+0.5)*0.4+0.2,
                    1.0)
    );
    vec3 color = vec3(texture(tex_color, frag_texcoord).rgb);
    color = (color_matrix * (1.0 - beat) + color_matrix2 * beat) * color;

    //color = color.b * vec3(1.0, 0.1, 0.1);
    //color = color.b * vec3(0.2, 0.2, 1.0);
    //color = color.b * vec3(0.5, 0.2, 1.0);
    //color = color.b * vec3(2.0, 1.0, 0.0);

    fg_color = vec4(color, 1.0);
    fg_position = vec4(texture(tex_position, frag_texcoord));
    fg_screen_position = vec4(texture(tex_screen_position, frag_texcoord));
}
