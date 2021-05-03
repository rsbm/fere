#version 330 core

uniform sampler2D u_tex0;
layout (location = 0) out vec4 io_color;

void main()
{
	vec2 q = gl_FragCoord.xy;
	ivec2 p = ivec2(int(q[0]), int(q[1]));

    vec3 value = texelFetch(u_tex0, p, 0).rgb;
    io_color = vec4(value, 1);
}



