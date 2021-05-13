#version 330 core
in vec3 io_pos;
in vec3 io_norm;
in vec2 io_tex;
in vec3 io_fnorm;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;

out vec3 wnormal;

void main()
{
    gl_Position = u_projection * u_view * u_model * vec4(io_pos, 1);
    wnormal = normalize(mat3(u_model) * io_norm);
}
