#version 410

// Bokeh disc shader, written by David Hoskins
// https://www.shadertoy.com/view/4d2Xzw

uniform sampler2D tex_position;
uniform sampler2D tex_screen_position;
uniform sampler2D tex_color;
uniform float time;
uniform float beat;

smooth in vec2 frag_texcoord;

out vec4 fg_position;
out vec4 fg_screen_position;
out vec4 fg_color;

// Bokeh disc.
// by David Hoskins.
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.

// The Golden Angle is (3.-sqrt(5.0))*PI radians, which doesn't precompiled for some reason.
// The compiler is a dunce I tells-ya!!
#define GOLDEN_ANGLE 2.39996

#define ITERATIONS 40

mat2 rot = mat2(cos(GOLDEN_ANGLE), sin(GOLDEN_ANGLE), -sin(GOLDEN_ANGLE), cos(GOLDEN_ANGLE));

//-------------------------------------------------------------------------------------------
vec3 Bokeh(sampler2D tex, vec2 uv, float radius)
{
    vec3 acc = vec3(0.0), div = acc;
    float r = 1.0;
    vec2 vangle = vec2(0.0,radius*0.01 / sqrt(float(ITERATIONS)));

    for (int j = 0; j < ITERATIONS; j++)
    {
        // the approx increase in the scale of sqrt(0, 1, 2, 3...)
        r += 1. / r;
        vangle = rot * vangle;
        vec3 col = texture(tex, uv + (r-1.0) * vangle).xyz; /// ... Sample the image
        vec3 bokeh = pow(col, vec3(4.0));
        acc += col * bokeh;
        div += bokeh;
    }
    return acc / div;
}

void main() {
    float blend = max(beat - 0.2, 0.0) * 0.7;
        // 0.0;
        // (sin(time) * 0.5 + 0.5) * 0.1;
        // (pow(1.0 - mod(time, 4.0) / 4.0, 8.0)) * 0.8;
    fg_color = vec4(Bokeh(tex_color, frag_texcoord, blend), 1.0);
    //fg_color = vec4(texture(tex_color, frag_texcoord));
    fg_position = vec4(texture(tex_position, frag_texcoord));
    fg_screen_position = vec4(texture(tex_screen_position, frag_texcoord));
}
