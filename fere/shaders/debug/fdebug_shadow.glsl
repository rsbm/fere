#version 330 core

uniform sampler2D u_tex0;
layout (location = 0) out vec4 io_color;

void main()
{
	vec2 q = gl_FragCoord.xy;
	ivec2 p = ivec2(int(q[0]), int(q[1]));

    vec3 value = texelFetch(u_tex0, p, 0).rgb;
    
    float r = sin(cos(value.r + 0.1) * 3.0 + 1.2) * 0.5 + 0.5;
    float g = sin(cos(value.g + 0.2) * 2.9 + 0.8) * 0.5 + 0.5;
    float b = sin(cos(value.b + 0.3) * 3.1 + 0.5) * 0.5 + 0.5;

    io_color = vec4(r, g, b, 1);
}



