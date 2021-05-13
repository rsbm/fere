#version 330 core
in vec3 io_pos;
in vec2 io_tex;

uniform mat4 u_model;
out vec2 uv;

void main()
{
    gl_Position = u_model * vec4(io_pos, 1);
    uv = io_tex;
}
