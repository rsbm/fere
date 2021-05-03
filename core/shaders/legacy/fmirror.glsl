#version 330 core
in vec3 wnormal;
in vec4 wpos;

uniform samplerCube u_tex0;

layout (location = 0) out vec4 io_color;

void main()
{
    vec3 fuck = wnormal;
    fuck *= -1;
    vec3 x = vec3(texture(u_tex0, fuck).rgb);
    io_color = vec4(x,1);
}
