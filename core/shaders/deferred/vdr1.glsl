#version 330 core
in vec3 io_pos;
in vec3 io_norm;
in vec2 io_tex;
in vec3 io_fnorm;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform bool u_inside;

out vec3 wnormal;
out vec3 wpos;
out vec2 uv;

void main()
{
    vec4 temp = u_model * vec4(io_pos, 1);
    gl_Position = u_projection * u_view * temp;

    wnormal = normalize(mat3(u_model) * io_norm);
    wnormal = u_inside ? -wnormal : wnormal;

    wpos = temp.xyz / temp.w;  
    uv = io_tex;
}
