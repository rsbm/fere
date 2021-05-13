#version 330 core
in vec3 wnormal;
in vec3 wpos;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform vec4 u_color;
uniform float u_metal;

layout (location = 0) out vec3 io_pos;
layout (location = 1) out vec3 io_norm;
layout (location = 2) out vec3 io_diffuse;
layout (location = 3) out float io_metal;

void main()
{
	io_pos = wpos;
    io_norm = normalize(wnormal);
    io_diffuse = u_color.xyz;
    io_metal = u_metal;
}
