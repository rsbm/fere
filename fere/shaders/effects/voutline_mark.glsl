#version 330 core
in vec3 io_pos;
uniform mat4 u_model;

void main()
{
    gl_Position = u_model * vec4(io_pos, 1);
}
