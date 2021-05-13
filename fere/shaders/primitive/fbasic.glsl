#version 330 core

uniform vec4 u_color;
in vec3 wnormal;

layout (location = 0) out vec4 io_color;

void main()
{
	io_color = u_color;
}
