#version 410

// GAUSS BLUR shader, written by mrharicot
// https://www.shadertoy.com/view/XdfGDH

uniform sampler2D tex_position;
uniform sampler2D tex_screen_position;
uniform sampler2D tex_color;
uniform float time;
uniform vec2 resolution;

#define iResolution resolution

smooth in vec2 frag_texcoord;

out vec4 fg_position;
out vec4 fg_screen_position;
out vec4 fg_color;

float normpdf(in float x, in float sigma)
{
    return 0.39894*exp(-0.5*x*x/(sigma*sigma))/sigma;
}


void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec3 c = texture(tex_color, fragCoord.xy / iResolution.xy).rgb;
    //declare stuff
    const int mSize = 3;
    const int kSize = (mSize-1)/2;
    float kernel[mSize];
    vec4 final_colour = vec4(0.0);

    //create the 1-D kernel
    float sigma = 7.0;
    float Z = 0.0;
    for (int j = 0; j <= kSize; ++j)
    {
        kernel[kSize+j] = kernel[kSize-j] = normpdf(float(j), sigma);
    }

    //get the normalization factor (as the gaussian has been clamped)
    for (int j = 0; j < mSize; ++j)
    {
        Z += kernel[j];
    }

    //read out the texels
    for (int i=-kSize; i <= kSize; ++i)
    {
        for (int j=-kSize; j <= kSize; ++j)
        {
            final_colour += kernel[kSize+j]*kernel[kSize+i]*texture(tex_color, (fragCoord.xy+vec2(float(i),float(j))) / iResolution.xy);
        }
    }

    fragColor = final_colour/(Z*Z);
}

float PI = 3.1415926;

void main() {
    float blend = 0.5; //sin(time) * 0.5 + 0.5;
    mainImage(fg_color, frag_texcoord * iResolution);
    vec4 alt_color = vec4(texture(tex_color, frag_texcoord));
    fg_color = fg_color * (1.0 - blend) + alt_color * blend;
    float fact = pow(cos(PI * (frag_texcoord.y - 0.5)), 0.5) * 0.8 + 0.2;
    fg_color = vec4(fg_color.rgb * fact, fg_color.a);
    fg_position = vec4(texture(tex_position, frag_texcoord));
    fg_screen_position = vec4(texture(tex_screen_position, frag_texcoord));
}
