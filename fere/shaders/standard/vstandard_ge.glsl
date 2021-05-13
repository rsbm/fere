#version 330 core
in vec3 io_pos;
in vec3 io_norm;
in vec2 io_tex;
in vec3 io_fnorm;

uniform ivec2 u_screen_size;
uniform bool u_gouraud_emission_on;
uniform vec3 u_cpos;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform mat3 u_model3;
uniform vec4 u_color;

uniform bool u_inside;

out vec3 wnormal;
out vec3 wpos;
out vec2 uv;
out vec3 gouraud_emisison;

/* ----- Material for the Gouraud emission ----- */
/*
texture binding
0 : basecolor
1 : roughness
2 : metalness
3 : emission
4 : emissionintensity
5 : normal
6 : alpha (not used here, only for forward)
*/
uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;
uniform sampler2D u_tex4;
uniform sampler2D u_tex5;
uniform sampler2D u_tex6;
uniform sampler2D u_tex7;

uniform vec3 u_basecolor;
uniform float u_roughness;
uniform float u_metalness;

uniform bool u_basecolor_on;
uniform bool u_roughness_on;
uniform bool u_metalness_on;

/* ----- G-Buffer of last frame for the Gouraud emission ----- */
/*
8(pos), 9(norm), 10(bc), 11(roughness),
 12(metalness), 13(emission), 14(shadow map), 15(lighting)
*/
uniform sampler2D u_tex8;
uniform sampler2D u_tex9;
uniform sampler2D u_tex10;
uniform sampler2D u_tex11;
uniform sampler2D u_tex12;
uniform sampler2D u_tex13;
uniform sampler2D u_tex14;
uniform sampler2D u_tex15;

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
    
	float attenuation = 1.0 / (dis * dis);
    vec3 radiance = lc * attenuation; 
	return (kD * bc / 3.141592 + specular) * radiance * NDL + em;
}

void main()
{
    vec4 temp = u_model * vec4(io_pos, 1);
    gl_Position = u_projection * u_view * temp;

    wnormal = normalize(u_model3 * io_norm);
    wnormal = u_inside ? -wnormal : wnormal;

    wpos = temp.xyz / temp.w;  
    uv = io_tex;

    // Gouraud Emission 
    vec3 emission = vec3(0);
    if (u_gouraud_emission_on)
    {
        ivec2 p;
        p = ivec2(int((gl_Position.x/gl_Position.w * 0.5 + 0.5)*u_screen_size.x),
        int((gl_Position.y/gl_Position.w * 0.5 + 0.5)*u_screen_size.y));

        vec3 basecolor = u_basecolor_on ? u_basecolor : texture(u_tex0, uv).rgb;
        float roughness = u_roughness_on ? u_roughness : texture(u_tex1, uv).r;
        float metalness = u_metalness_on ? u_metalness : texture(u_tex2, uv).r;

        for(int i = -5; i <= 5; i++)
        {
            for(int j = -5; j <= 5; j++)
            {
                ivec2 q = p + ivec2(i, j) * 20;

                vec3 wpos_sample = texelFetch(u_tex8, p, 0).rgb;
                vec3 wnormal_sample = normalize(texelFetch(u_tex9, p, 0).rgb);
                vec3 em_sample = texelFetch(u_tex13, p, 0).rgb;

                emission += em_sample;
                continue;

                emission += shade_pbr(wpos, wnormal, u_cpos, vec4(wpos_sample, 1),
                em_sample * max(0.0, dot(wnormal_sample, wnormal)),
                basecolor, roughness, metalness, vec3(0), 1.0);
            }
        }
    }
    
    gouraud_emisison = emission;
    gouraud_emisison = texture(u_tex13, vec2(gl_Position)).rgb;
}
