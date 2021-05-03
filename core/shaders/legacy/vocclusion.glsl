#version 330 core
in vec3 io_pos;
in vec3 io_norm;
in vec2 io_tex;
in vec3 io_fnorm;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform vec4 u_color;

uniform bool u_inside;

out vec3 rpos;

struct SLightVolume
{
	mat4 rtrans; // room-space transformation 

    vec3 cellsize;
    ivec3 nums;

    sampler3D tex;
};
uniform SLightVolume u_lv;

void main()
{
    vec4 temp = u_model * vec4(io_pos, 1);
    gl_Position = u_projection * u_view * temp;

    rpos = io_norm;  
}
