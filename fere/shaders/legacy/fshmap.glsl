#version 330 core
in vec3 vnormal;
in vec3 vpos;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform vec4 u_color;

uniform vec3 u_material;

struct SLight // could be either point or directional
{
	vec4 vpos;
	vec3 color;
	mat4 trans;
};
uniform SLight u_lights[3];
uniform vec3 u_ambient;

layout (location = 0) out vec4 io_color;

void main()
{
	io_color = u_color;
	
    io_color[3] = gl_FragCoord[2];
}
