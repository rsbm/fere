#version 330 core
in vec3 wnormal;
in vec3 wpos;

uniform int u_lighting;
uniform vec3 u_ambient;
uniform float u_metal;
uniform vec4 u_color;
uniform vec3 u_cpos;

struct SLight // could be either point or directional
{
	vec4 wpos;
	vec3 color;
	mat4 trans;
};
uniform SLight u_lights[1];
layout (location = 0) out vec4 io_color;

//pos : position of pixel | norm : normal of pixel | cpos : camera position
//lpos : light position | color : light color | df : diffuse color | sp : specular constant
vec3 light(vec3 pos, vec3 norm, vec3 cpos, vec4 lpos, vec3 color, vec3 df, float sp)
{
	vec3 L = lpos.xyz - pos;
	float dis = length(L);
	float katt = 1.0 /(1 +  3 * dis);
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
	vec3 total = u_ambient * vec3(u_color);
	for(int i = 0; i < 1; i++)
	{
		total += light(wpos, wnormal, u_cpos, u_lights[0].wpos, u_lights[0].color, vec3(u_color), u_metal);
	}
	// this is forward rendering
	// lighting is just only effect.
	// so we guaratee minimum value (0.7)
	total *= 0.3; total += vec3(0.7);
	io_color = vec4(total, 1) * u_color;
	io_color = clamp(io_color, 0.0, 1.0);
}
