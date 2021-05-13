#version 330 core
uniform usampler2D u_tex0;
uniform ivec3 u_basecolor;
uniform float u_alpha;

layout (location = 0) out vec4 io_color;

void main()
{
	vec2 q = gl_FragCoord.xy;
	ivec2 p = ivec2(int(q[0]), int(q[1]));
    uint check = texelFetch(u_tex0, p, 0)[0];
    if (check == uint(1)) discard;
	io_color = vec4(vec3(u_basecolor)/255.0, u_alpha);
}



