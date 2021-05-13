#version 330 core
in vec3 wnormal;
in vec3 wpos;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform vec4 u_color;

layout (location = 0) out vec3 io_col;
layout (location = 1) out vec3 io_pos;
layout (location = 2) out vec3 io_norm;

void main()
{
    io_col = u_color.xyz;
	io_pos = wpos;
    io_norm = wnormal;
}
