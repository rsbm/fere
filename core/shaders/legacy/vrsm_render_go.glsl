#version 330 core
in vec3 io_pos;
in vec3 io_norm;
in vec2 io_tex;
in vec3 io_fnorm;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform vec4 u_color;

uniform bool u_inside;
uniform vec3 u_material;


out vec3 vnormal;
out vec3 wnormal;
out vec3 wpos;
out vec3 vpos;

out vec3 color_rsm;

struct SLight // could be either point or directional
{
	vec4 vpos;
	vec3 color;
	mat4 trans;
};
uniform SLight u_lights[3];

uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;

vec3 vnormal_n;
vec3 wnormal_n;

vec3 light(vec3 pos, vec3 norm, vec4 lpos, vec3 color, float f, bool spec = true)
{
	if(f == 0)
		return vec3(0);

	vec3 L = lpos[3] == 0 ? lpos.xyz : lpos.xyz - pos;
	float dis = length(L);
	float katt = lpos[3] == 0 ? 1 : 8.0/(dis);
	L = normalize(L);
	vec3 result; 
	
	//diffuse
	result = f * color * 
	u_material[1] * katt * max(dot(norm,L), 0.0);
		
	return result;
}
float rand(vec2 co)
{
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}
vec3 rsm(int q)
{
	vec4 temp = u_lights[q].trans * vec4(wpos, 1);
	vec3 pc = temp.xyz/temp.w;
	pc = pc * 0.5 + 0.5;
	vec3 total = vec3(0);
	
    int n = 20;
    for(int i = 0; i < n; i++)
        for(int j = 0; j < n; j++)
        {
            vec2 coord = vec2(float(i) / float(n-1), float(j) / float(n-1));
            vec3 scol = texture(u_tex1, coord).rgb;
            vec3 spos = texture(u_tex2, coord).rgb;
            vec3 snorm = texture(u_tex3, coord).rgb;
            float f = dot(normalize(snorm), normalize(wpos-spos));
            f = max(f,0);

            //diffuse only
            vec3 a = light(wpos, wnormal_n, vec4(spos,1), scol, 2.2 / n / n * f, false);
            total += clamp(a, 0.0, 1.0);
        }
    /*
	int n = 500;
	for(int i = 0; i < n; i++)
	{	
		float xx = rand(vec2(float(i), pc.x * pc.y * 0.9));
		float yy = rand(vec2(float(i), pc.y * pc.x));
		vec2 coord = vec2(xx, yy);
		vec3 scol = texture(u_tex1, coord).rgb;
		vec3 spos = texture(u_tex2, coord).rgb;
		vec3 snorm = texture(u_tex3, coord).rgb;
		float f = dot(normalize(snorm), normalize(wpos-spos));
		f = max(f,0);

		//diffuse only
		vec3 a = light(wpos, wnormal_n, vec4(spos,1), scol, 1.2 / n * f, false);
		total += clamp(a, 0.0, 1.0);
	}
    */
	return total;
	
}


void main()
{
    vec4 temp = u_model * vec4(io_pos, 1);
    wpos = temp.xyz;  
    temp = u_view * temp;
    vpos = temp.xyz;
    gl_Position = u_projection * temp;

    wnormal = normalize(mat3(u_model) * io_fnorm);
    wnormal = u_inside ? -wnormal : wnormal;
    vnormal = mat3(u_view) * wnormal;

    vnormal_n = normalize(vnormal);
    wnormal_n = normalize(wnormal);

    color_rsm = rsm(0);
}
