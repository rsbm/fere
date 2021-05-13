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

out vec3 vnormal;
out vec3 wnormal;
out vec3 wpos;
out vec3 vpos;

void main()
{
    vec4 temp = u_model * vec4(io_pos, 1);
    wpos = temp.xyz;  
    temp = u_view * temp;
    vpos = temp.xyz;
    gl_Position = u_projection * temp;

    wnormal = normalize(mat3(u_model) * io_fnorm);
    wnormal = u_inside ? -wnormal : wnormal;
    vnormal = mat3(u_view) * wnormal;
}
