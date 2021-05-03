#version 330 core
in vec3 vnormal;
in vec4 vpos;
in vec2 uv;

uniform uint u_mark;

layout (location = 0) out uint io_color;

void main()
{
    io_color = u_mark;
}
