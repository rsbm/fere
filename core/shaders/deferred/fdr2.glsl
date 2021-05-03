#version 330 core
struct SLight // could be either point or directional
{
	vec4 wpos;
	vec3 color;
	mat4 trans;
	bool round;
	float smoothness;
};
uniform SLight u_lights[1];
uniform vec3 u_cpos;
uniform int u_shadow;
/*
0(pos), 1(norm), 2(bc), 3(roughness), 4(metalness), 5(emission), 6(shadow map), 7(lighting)
*/

uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;
uniform sampler2D u_tex4;
uniform sampler2D u_tex5;
uniform sampler2D u_tex6;
uniform isampler2D u_tex7;

layout (location = 0) out vec4 io_color;


// P : position of fragment
float shade_shadow(vec3 P, vec3 N) {
	if (u_shadow == 0) return 1;

	vec2 texel_size = 1.0 / textureSize(u_tex6, 0);

	// lies in clipping space of light's eye
	vec4 tpos = u_lights[0].trans * vec4(P, 1);
	vec3 pc = tpos.xyz/tpos.w;

	float weight = 1.0;
	if (u_lights[0].round) 
	{
		float dis = pc.x*pc.x + pc.y*pc.y;
		if (dis > 1)
			return 0;
		weight = 1 - pow(dis, 1 + u_lights[0].smoothness * 10); 
	}
	else 
	{
		if(abs(pc.x) >= 1.0 || abs(pc.y) >= 1.0) 
			return 0;
		if(pc.z <= 0)
			return 0;
	}
	pc = pc * 0.5 + 0.5;

	float shadow = 0;
	vec3 L = normalize(u_lights[0].wpos.xyz - P);

	for (int i = -2; i <= 2; i++)
	{
		for(int j = -2; j <= 2; j++)
		{
			float depth = texture(u_tex6, pc.xy + ivec2(i,j) * texel_size).r;
			float bias = max(0.001 * (1.0) - dot(N, L), 0.0001);
			shadow += pc.z - bias > depth ? 0.0 : 1.0;
		}
	}

	return shadow / 25.0;
}

// PBR formula
vec3 fresnelSchlick(float cosTheta, vec3 F0)
{
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
} 
float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a      = roughness*roughness;
    float a2     = a*a;
    float NdotH  = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;
	
    float num   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = 3.141592 * denom * denom;
    return num / denom;
}
float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float num   = NdotV;
    float denom = NdotV * (1.0 - k) + k;
	
    return num / denom;
}
float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2  = GeometrySchlickGGX(NdotV, roughness);
    float ggx1  = GeometrySchlickGGX(NdotL, roughness);
	
    return ggx1 * ggx2;
}

// P : position of fragment | N : normal of fragment | P_c : camera position
// P_l : light position | lc : light color 
// bc : basecolor | ro : roughness | mt : metalness | em : emission | sd : shadow
vec3 shade_pbr(vec3 P, vec3 N, vec3 P_c, vec4 P_l, vec3 lc,
 vec3 bc, float ro, float mt, vec3 em, float sd)
{
	vec3 V = normalize(P_c - P);
	vec3 L = normalize(P_l.xyz - P); 
	vec3 H = normalize(V + L);

	lc *= sd;
	
	float dis = length(P_l.xyz - P);
	float NDL = dot(N, L);
	float NDV = dot(N, V);

	if(NDL <= 0.0 || NDV <= 0.0) return vec3(0);

	float NDF = DistributionGGX(N, H, ro);
	float G = GeometrySmith(N, V, L, ro);

	vec3 F0 = vec3(0.04); 
	F0 = mix(F0, bc, mt);
	vec3 F = fresnelSchlick(NDL, F0);
	
	vec3 kS = F;
    vec3 kD = vec3(1.0) - kS;
    kD *= 1.0 - mt;

	vec3 numerator    = NDF * G * F;
	float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
	vec3 specular     = numerator / max(denominator, 0.001);  
    
	float attenuation = 5.0 / (dis * dis);
    vec3 radiance = lc * attenuation; 
	return (kD * bc / 3.141592 + specular) * radiance * NDL + em;
}

void main()
{
	vec2 q = gl_FragCoord.xy;
	ivec2 p = ivec2(int(q[0]), int(q[1]));

    vec3 wpos = texelFetch(u_tex0, p, 0).rgb;
    vec3 wnormal = normalize(texelFetch(u_tex1, p, 0).rgb);

    vec3 bc = texelFetch(u_tex2, p, 0).rgb;
    float ro = texelFetch(u_tex3, p, 0).r;
	float mt = texelFetch(u_tex4, p, 0).r;
	vec3 em = texelFetch(u_tex5, p, 0).rgb;
	int lighting = texelFetch(u_tex7, p, 0).r;

	//io_color = vec4(1, 0, 0, 1);
	//return;

	if(lighting == 1)
	{
		float shadow = shade_shadow(wpos, wnormal);

		if (shadow < 0.0001)
		{
			io_color = vec4(0,0,0,1);
			return;
		}

		vec3 res = shade_pbr(wpos, wnormal, u_cpos, 
		u_lights[0].wpos, u_lights[0].color, bc, ro, mt, em, shadow);

		res = res / (res + vec3(1.0));
		io_color = vec4(res, 1.0);
	}
	else {
		discard;
	};

}



