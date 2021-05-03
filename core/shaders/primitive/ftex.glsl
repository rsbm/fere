#version 330 core
in vec2 uv;

uniform sampler2D u_tex0;

layout (location = 0) out vec4 io_color;

void main()
{
    vec3 x = vec3(texture(u_tex0, uv).rgb);
    io_color = vec4(x, 1);
}
