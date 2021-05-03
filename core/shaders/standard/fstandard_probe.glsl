#version 330 core
in vec3 wnormal;
in vec3 wpos;
in vec2 uv;
in vec3 gouraud_emisison;

uniform vec3 u_basecolor;
uniform float u_roughness;
uniform float u_metalness;
uniform vec3 u_emission;
uniform float u_emission_intensity;
//there is no normal uniform (think about it)
uniform vec3 u_emission2;
uniform float u_emission_intensity2;

uniform bool u_basecolor_on;
uniform bool u_roughness_on;
uniform bool u_metalness_on;
uniform bool u_emission_on;
uniform bool u_emission_intensity_on;
uniform bool u_normal_on;

uniform bool u_emission_blend; // emission 2 is optional. (for blending)
uniform float u_emission_blend_rate; // 0 ~ 1
uniform bool u_emission2_on;
uniform bool u_emission_intensity2_on;

uniform uint u_object_index;
uniform int u_lighting;

/*
texture binding
0 : basecolor
1 : roughness
2 : metalness
3 : emission
4 : emissionintensity
5 : normal
6 : alpha (not used here, only for forward)
*/

// Note : if the texture is normalized-int type, then we can use sampler2D, not isampler2D
uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;
uniform sampler2D u_tex4;
uniform sampler2D u_tex5;
uniform sampler2D u_tex6;
uniform sampler2D u_tex7;

layout (location = 0) out vec3 io_diffuse;
layout (location = 1) out vec3 io_emission;

/// intensity: 0(0), 0.1(1.0), 1.0(256.0)
/// final emission: range from 0.0 to 256.0
/// contributes to GI after 32.0 
float intenstiy(float i) {
    return i*i*256.0;
}

void main()
{
    vec3 basecolor = u_basecolor_on ? u_basecolor : texture(u_tex0, uv).rgb;
    io_diffuse = basecolor;

    vec3 em = u_emission_on ? u_emission : texture(u_tex3, uv).rgb;
    float ei = u_emission_intensity_on ? u_emission_intensity : texture(u_tex4, uv).r;   

    if (u_emission_blend) // emission blending
    {
        vec3 em2 = u_emission_on ? u_emission2 : texture(u_tex6, uv).rgb;
        float ei2 = u_emission_intensity2_on ? u_emission_intensity2 : texture(u_tex7, uv).r;   
        io_emission = em * intenstiy(ei) * (1 - u_emission_blend_rate) + em2 * intenstiy(ei2) * u_emission_blend_rate;
    }
    else io_emission = em * intenstiy(ei);
}
