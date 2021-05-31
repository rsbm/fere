#version 330 core

in vec2 uv;

uniform sampler2D u_tex0;
layout (location = 0) out vec4 io_color;

void main()
{
    vec3 value = texture(u_tex0, uv).rgb;
    if value == vec3(0) {
        io_color = vec4(0, 0, 0, 1);
    } else {
        io_color = value / length(value);
    }
    
}
