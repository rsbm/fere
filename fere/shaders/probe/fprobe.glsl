#version 330 core
in vec3 wnormal;
in vec3 wpos;
in vec2 uv;

uniform vec3 u_basecolor;
uniform vec3 u_emission;
uniform float u_emission_intensity;

uniform bool u_basecolor_on;
uniform bool u_emission_on;
uniform bool u_emission_intensity_on;

/*
texture binding
0 : basecolor
1 : emission
2 : emissionintensity
*/
uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;

layout (location = 0) out vec3 io_pos;
layout (location = 1) out vec3 io_norm;
layout (location = 2) out vec3 io_basecolor;
layout (location = 3) out vec3 io_emission;

void main()
{
	io_pos = wpos;
    io_norm = normalize(wnormal);
    io_basecolor = u_basecolor_on ? u_basecolor : texture(u_tex0, uv).rgb;
    vec3 em = u_emission_on ? u_emission : texture(u_tex1, uv).rgb;
    float ei = u_emission_intensity_on ? u_emission_intensity : texture(u_tex2, uv).r;   
    io_emission = em * ei;
}
