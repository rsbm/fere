#version 330 core

uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;
uniform sampler2D u_tex4;
uniform sampler2D u_tex5;
uniform sampler2D u_tex6;
uniform isampler2D u_tex7;

uniform vec3 u_ambient;
layout (location = 0) out vec4 io_color;

void main()
{
	vec2 q = gl_FragCoord.xy;
	ivec2 p = ivec2(int(q[0]), int(q[1]));

    vec3 wpos = texelFetch(u_tex0, p, 0).rgb;
    vec3 wnormal = texelFetch(u_tex1, p, 0).rgb;
 
    vec3 bc = texelFetch(u_tex2, p, 0).rgb;
    float ro = texelFetch(u_tex3, p, 0).r;
	float mt = texelFetch(u_tex4, p, 0).r;
	vec3 em = texelFetch(u_tex5, p, 0).rgb;
	int lighting = texelFetch(u_tex7, p, 0).r;

	if(lighting == 0)
	{
		discard; 
	}
	else if(lighting == 1)
	{
		io_color = vec4(bc * u_ambient, 1);
	}
	else if (lighting == 2 || lighting == 4)
	{
		io_color = vec4(bc, 1);
	}
	else discard;
}



