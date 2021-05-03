#version 330 core
in vec3 wnormal;
in vec3 wpos;
in vec2 uv;

uniform vec3 u_basecolor;
uniform vec3 u_emission;
uniform float u_emissionintensity;
uniform vec3 u_emission2;
uniform float u_emissionintensity2;

uniform bool u_basecolor_on;
uniform bool u_emission_on;
uniform bool u_emissionintensity_on;
uniform bool u_emission2_on;
uniform bool u_emissionintensity2_on;

uniform bool u_emission2_enable; // emission 2 is optional. (for blending)
uniform float u_emission_blending; // 0 ~ 1

uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;
uniform sampler2D u_tex4;
uniform sampler2D u_tex5;
uniform sampler2D u_tex6;
uniform sampler2D u_tex7;

layout (location = 0) out vec3 io_pos;
layout (location = 1) out vec3 io_normal;
layout (location = 2) out vec3 io_basecolor;
layout (location = 3) out vec3 io_emission;

void main()
{
    io_pos = wpos;
    io_normal = wnormal;
	io_basecolor = u_basecolor_on ? u_basecolor : texture(u_tex0, uv).rgb;
    
    vec3 em = u_emission_on ? u_emission : texture(u_tex3, uv).rgb;
    float ei = u_emissionintensity_on ? u_emissionintensity : texture(u_tex4, uv).r;   

    if (u_emission2_enable) // emission blending
    {
        vec3 em2 = u_emission_on ? u_emission2 : texture(u_tex6, uv).rgb;
        float ei2 = u_emissionintensity2_on ? u_emissionintensity2 : texture(u_tex7, uv).r;   
        io_emission = em * ei * (1 - u_emission_blending) + em2 * ei2 * u_emission_blending;
    }
    else io_emission = em * ei;

    io_emission = u_emission;
}
