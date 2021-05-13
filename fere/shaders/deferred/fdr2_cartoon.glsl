#version 430 core
struct SLight // could be either point or directional
{
	vec4 wpos;
	vec3 color;
	mat4 trans;
};
uniform SLight u_lights[1];
uniform vec3 u_cpos;

uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;
uniform sampler2D u_tex4;
uniform sampler2D u_tex5;

layout (location = 0) out vec4 io_color;


//pos : position of pixel | norm : normal of pixel | cpos : camera position
//lpos : light position | color : light color | df : diffuse color | sp : specular constant
vec3 light(vec3 pos, vec3 norm, vec3 cpos, vec4 lpos, vec3 color, vec3 df, float sp)
{
	vec3 L = lpos.xyz - pos;
	float dis = length(L);
	float katt = 1.0 /(1 +  3 * dis + dis*dis);
	L = normalize(L);
	vec3 result = vec3(0); 
	
	//diffuse
	result += df * color * katt * max(dot(norm,L), 0.0);

    //specular
	vec3 E = normalize(cpos - pos);
	vec3 R = normalize(-reflect(L,norm));
	result += sp * color * katt * pow(max(dot(R,E), 0.0), 2);

	return result;
}

void main()
{
	vec2 q = gl_FragCoord.xy;
	ivec2 p = ivec2(int(q[0]), int(q[1]));

    vec3 wpos = texelFetch(u_tex0, p, 0).rgb;
    vec3 wnormal = texelFetch(u_tex1, p, 0).rgb;
    vec3 df = texelFetch(u_tex2, p, 0).rgb;
    float metal = texelFetch(u_tex3, p, 0).r;
	vec3 fl = texelFetch(u_tex4, p, 0).rgb;

    vec3 total = fl;
    total += light(wpos, wnormal, u_cpos, u_lights[0].wpos, u_lights[0].color, df, metal);

    float sobel[5][5] = {{1, 4, 6, 4, 1}, {2, 8, 12, 8, 2,}, {0, 0, 0, 0, 0}, {-2, -8, -12, -8, -2}, {-1, -4, -6, -4, -1}};
    vec3 wpos_sobel_sum_x = vec3(0);
    vec3 wnor_sobel_sum_x = vec3(0);
    vec3 wpos_sobel_sum_y = vec3(0);
    vec3 wnor_sobel_sum_y = vec3(0);

    for(int dx = -2; dx <= 2; dx++)
    {
        for(int dy = -2; dy <= 2; dy++)
        {
            vec3 pos_s = texelFetch(u_tex0, p + ivec2(dx, dy), 0).rgb;
            vec3 nor_s = texelFetch(u_tex1, p + ivec2(dx, dy), 0).rgb;

            wpos_sobel_sum_x += sobel[dx + 2][dy + 2] * pos_s;
            wnor_sobel_sum_x += sobel[dx + 2][dy + 2] * nor_s;
            wpos_sobel_sum_y += sobel[dy + 2][dx + 2] * pos_s;
            wnor_sobel_sum_y += sobel[dy + 2][dx + 2] * nor_s;
        }
    }
        
    float depth = (texelFetch(u_tex5, p, 0).r - 0.99)*100;

    float g = (length(wpos_sobel_sum_x) + length(wpos_sobel_sum_y)) / (1000.0 * depth);
    float b = (length(wnor_sobel_sum_x) + length(wnor_sobel_sum_y)) / 100.0;
    g = clamp(g, 0.0, 1.0);
    b = clamp(b, 0.0, 1.0);

    vec4 res = clamp(vec4(total, 1), 0.0, 1.0);
    if(b >= 0.5)
    {
        float bb = (g * b - 0.25) /0.75;
        res = res * (bb - 1) + vec4(0,0,0,1) * bb;
    }

	io_color = res;
}



