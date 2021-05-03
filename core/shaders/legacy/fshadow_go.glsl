#version 330 core
in vec3 vnormal;
in vec3 wnormal;
in vec3 vpos;
in vec3 wpos;
in vec3 color_rsm;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform vec4 u_color;

uniform vec3 u_material;

struct SLight // could be either point or directional
{
	vec4 vpos;
	vec3 color;
	mat4 trans;
};
struct S2vec
{
	vec3 a, b;
};
uniform SLight u_lights[3];
uniform vec3 u_ambient;

uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;

layout (location = 0) out vec4 io_color;

//////


vec3 vnormal_n = normalize(vnormal);
vec3 wnormal_n = normalize(wnormal);



float shadow(int i)
{
	vec4 temp = u_lights[i].trans * vec4(wpos, 1);
	vec3 pc = temp.xyz/temp.w;
	pc = pc * 0.5 + 0.5;
	if(pc.x < 0 || pc.y < 0 || pc.x > 1 || pc.y > 1 || pc.z > 1)
		return 0;

	float fuck = 1.0/pow(0.5, 4);
	float aa = 1 - fuck *  pow(abs(pc.x-0.5),4);
	aa *= 1 - fuck *  pow(abs(pc.y-0.5),4);


	float depth = texture(u_tex0, pc.xy).r;
	vec3 L = u_lights[i].vpos[3] == 0 ?
	u_lights[i].vpos.xyz : u_lights[i].vpos.xyz - vpos;
	float bias = max(0.01 * (1.0) - dot(vnormal_n, L),0.001) ;
	float shadow = pc.z - bias > depth ? 0.0 : 1.0;
	return shadow;
}

S2vec light(vec3 pos, vec3 norm, vec4 lpos, vec3 color, float f, bool spec = true)
{
	if(f == 0)
	{
		S2vec result;
		result.a = vec3(0);
		result.b = vec3(0);
		return result;
	}
	vec3 L = lpos[3] == 0 ? lpos.xyz : lpos.xyz - pos;
	float dis = length(L);
	float katt = lpos[3] == 0 ? 1 : 8.0/(dis);
	L = normalize(L);
	S2vec result; 
	
	//diffuse
	result.a = f * color * 
	u_material[1] * katt * max(dot(norm,L), 0.0);
		
	//specular
	if(spec)
	{
		vec3 E = normalize(-vpos);
		vec3 R = normalize(-reflect(L,norm));
		result.b = f * color * 
		u_material[2] * katt * pow(max(dot(R,E), 0.0), 0.8);
	}	
	else
		result.b = vec3(0);

	return result;
}

void main()
{
	vec3 total = u_ambient;
	for(int i = 0; i < 3; i++)
	{
		S2vec r = light(vpos, vnormal_n, u_lights[i].vpos, u_lights[i].color, 1);
		float s = shadow(0);
		total += clamp(r.a * s, 0.0, 1.0) + clamp(r.b * s, 0.0, 0.5);
	}
	
	total += color_rsm;

	io_color = vec4(total, 1) * u_color;
	io_color = clamp(io_color, 0.0, 1.0);
}



