#version 330 core

uniform isampler2D u_tex0;
layout (location = 0) out vec4 io_color;

void main()
{
	vec2 q = gl_FragCoord.xy;
	ivec2 p = ivec2(int(q[0]), int(q[1]));

    int value = texelFetch(u_tex0, p, 0).r;
    float fuck = value == 2 ? 0.0 : 1.0;
    io_color = vec4(fuck, 0, 0, 1);
}



