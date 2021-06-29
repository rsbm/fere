#version 330 core
in vec2 uv;
uniform vec4 u_color;

uniform sampler2D u_tex0;

layout (location = 0) out vec4 io_color;

void main()
{
    io_color = texture(u_tex0, uv).rgba * u_color;
}
