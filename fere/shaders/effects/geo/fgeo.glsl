#version 330 core

uniform vec4 u_color;
uniform vec3 u_cpos;

in vec3 wnormal;
in vec3 wpos;

layout (location = 0) out vec4 io_color;

void main()
{
    vec3 dir = normalize((normalize(u_cpos - wpos) + normalize(u_cpos)) / 2.0);
	io_color = vec4(vec3(u_color) * dot(wnormal, dir), u_color.w);
}