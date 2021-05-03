#version 330 core
in vec3 io_pos;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;

void main()
{
    vec4 temp = u_model * vec4(io_pos, 1);
    gl_Position = u_projection * u_view * temp;
}
