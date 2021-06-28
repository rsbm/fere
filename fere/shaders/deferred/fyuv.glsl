#version 330 core

uniform sampler2D u_tex0;
uniform vec4 u_color;

layout (location = 0) out uint out_y;
layout (location = 1) out uint out_cb;
layout (location = 2) out uint out_cr;

void main()
{
	vec2 q = gl_FragCoord.xy; 
	ivec2 p = ivec2(int(q[0]), int(q[1]));
    vec3 result = texelFetch(u_tex0, p, 0).rgb;
    result *= 255;

    float y = 0.299 * result[0] + 0.587 * result[1] + 0.114 * result[2];
    float cb = (result[2] - y) * 0.564 + 128.0;
    float cr = (result[0] - y) * 0.713 + 128.0;

    out_y = int(y);
    out_cb = int(cb);
    out_cr = int(cr);
}
