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

uniform vec2 u_texcoord0;
uniform vec2 u_texcoord1;

out vec2 uv;

void main()
{
    vec4 temp = u_view * u_model * vec4(io_pos, 1);
    gl_Position = u_projection * temp;
    uv = io_tex * (u_texcoord1 - u_texcoord0) + u_texcoord0;
}
