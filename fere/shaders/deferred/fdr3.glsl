#version 330 core

uniform sampler2D u_tex0;
uniform vec4 u_color;

layout (location = 0) out vec4 io_color;
void main()
{
	vec2 q = gl_FragCoord.xy; 
	ivec2 p = ivec2(int(q[0]), int(q[1]));
    vec3 result = texelFetch(u_tex0, p, 0).rgb;
	io_color = clamp(vec4(result, 1), 0.0, 1.0);
}



