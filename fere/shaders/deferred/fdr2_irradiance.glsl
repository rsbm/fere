#version 330 core
struct SLight // could be either point or directional
{
	vec4 wpos;
	vec3 color;
	mat4 trans;
	bool round;
	float smoothness;
};

struct SLightVolume
{
    /// World -> Chamber transformation
	mat4 trans;
	
	/// Chamber coordinate of probe (1,1,1) where (0,0,0) is the padded one
	vec3 offset;

    vec3 cell_size;
    ivec3 nums; // with padding
	vec3 padded_room_size; // cell_size * nums
    vec3 room_size;

	int params; // sh dimension
};
uniform SLightVolume u_lv;

/*
0(pos), 1(norm), 2(bc), 3(roughness), 4(metalness), 5(emission), 6(_), 7(lighting)
8(sh_illumination) 9(sh_depth)
*/

uniform sampler2D u_tex0;
uniform sampler2D u_tex1;
uniform sampler2D u_tex2;
uniform sampler2D u_tex3;
uniform sampler2D u_tex4;
uniform sampler2D u_tex5;
uniform sampler2D u_tex6;
uniform isampler2D u_tex7;

uniform sampler3D u_tex8;
uniform sampler3D u_tex9;

layout (location = 0) out vec4 io_color;

vec3 linear(float x, vec3 p0, vec3 p1)
{
	return p0 * (1 - x) + p1 * x;
}

vec3 bilinear(float x, float y, vec3 p00, vec3 p10, vec3 p01, vec3 p11)
{
	return linear(y, linear(x, p00, p10), linear(x, p01, p11));
}

vec3 trilinear(float x, float y, float z,
vec3 p000, vec3 p100, vec3 p010, vec3 p110, vec3 p001, vec3 p101, vec3 p011, vec3 p111)
{
	vec3 b1 = bilinear(x, y, p000, p100, p010, p110);
	vec3 b2 = bilinear(x, y, p001, p101, p011, p111);
	return linear(z, b1, b2);
}

float linear_1(float x, float p0, float p1)
{
	return p0 * (1 - x) + p1 * x;
}

float bilinear_1(float x, float y, float p00, float p10, float p01, float p11)
{
	return linear_1(y, linear_1(x, p00, p10), linear_1(x, p01, p11));
}

float trilinear_1(float x, float y, float z,
float p000, float p100, float p010, float p110, float p001, float p101, float p011, float p111)
{
	float b1 = bilinear_1(x, y, p000, p100, p010, p110);
	float b2 = bilinear_1(x, y, p001, p101, p011, p111);
	return linear_1(z, b1, b2);
}

vec3 spherical_harmonics(vec3 sh_coeff[18], vec3 p, mat3 table)
{
    vec3 result = vec3(0);

    result += sh_coeff[0];

    result += sh_coeff[1] * p[1];
    result += sh_coeff[2] * p[2];
    result += sh_coeff[3] * p[0];

    result += sh_coeff[4] * table[0][1];
    result += sh_coeff[5] * table[1][2];
    result += sh_coeff[6] * (3 * table[2][2] - 1);
    result += sh_coeff[7] * table[0][2];
    result += sh_coeff[8] * (table[0][0] - table[1][1]);

	if(u_lv.params <= 9) return result;

    result += sh_coeff[9] * table[0][1] * (table[0][0] - table[1][1]);
    result += sh_coeff[10] * table[1][2] * (3 * table[0][0] - table[1][1]);
    result += sh_coeff[11] * table[0][1] * (7 * table[2][2] - 1);
    result += sh_coeff[12] * table[1][2] * (7 * table[2][2] - 3);
    result += sh_coeff[13] * table[2][2] * ((35 * table[2][2] - 30) + 3);
    result += sh_coeff[14] * table[0][2] * (7 * table[2][2] - 3);
    result += sh_coeff[15] * (table[0][0] - table[1][1]) * (7 * table[2][2] - 1);
    result += sh_coeff[16] * table[0][2] * (table[0][0] - 3 * table[1][1]);
    result += sh_coeff[17] * (table[0][0] * (table[0][0] - 3 * table[1][1]) - table[1][1] * (3 * table[0][0] - table[1][1]));

	return result;
}

float spherical_harmonics_1(float sh_coeff[18], vec3 p, mat3 table)
{
    float result = 0;

    result += sh_coeff[0];

    result += sh_coeff[1] * p[1];
    result += sh_coeff[2] * p[2];
    result += sh_coeff[3] * p[0];

    result += sh_coeff[4] * table[0][1];
    result += sh_coeff[5] * table[1][2];
    result += sh_coeff[6] * (3 * table[2][2] - 1);
    result += sh_coeff[7] * table[0][2];
    result += sh_coeff[8] * (table[0][0] - table[1][1]);

	if(u_lv.params <= 9) return result;

    result += sh_coeff[9] * table[0][1] * (table[0][0] - table[1][1]);
	result += sh_coeff[10] * table[1][2] * (3 * table[0][0] - table[1][1]);
    result += sh_coeff[11] * table[0][1] * (7 * table[2][2] - 1);
    result += sh_coeff[12] * table[1][2] * (7 * table[2][2] - 3);
	result += sh_coeff[13] * (table[2][2] * (35 * table[2][2] - 30) + 3);
	result += sh_coeff[14] * table[0][2] * (7 * table[2][2] - 3);
    result += sh_coeff[15] * (table[0][0] - table[1][1]) * (7 * table[2][2] - 1);
    result += sh_coeff[16] * table[0][2] * (table[0][0] - 3 * table[1][1]);
    result += sh_coeff[17] * (table[0][0] * (table[0][0] - 3 * table[1][1]) - table[1][1] * (3 * table[0][0] - table[1][1]));

	return result;
}

void irradiance_volume(vec3 P, vec3 N, out vec3 illumination, out float depth)
{
    vec4 P_ = u_lv.trans * vec4(P, 1);
    P = vec3(P_) / P_.w;
    P.x += 0.5 * u_lv.room_size.x;
    P.y += 0.5 * u_lv.room_size.y;
	P += u_lv.cell_size - u_lv.offset;
	P /= u_lv.padded_room_size;
	P *= (u_lv.nums + ivec3(1,1,1));

	ivec3 p = ivec3(P); // lowest x,y,z probe
	vec3 w = P - vec3(p);

	vec3 sh_illumination[18];
	float sh_depth[18];

	ivec3 param_step = ivec3(0, 0, u_lv.nums[2] + 2);
	for(int i = 0; i < u_lv.params; i++)
	{
		vec3 illu[8];
		illu[0] = texelFetch(u_tex8, p + ivec3(0, 0, 0) + param_step * i, 0).rgb;
		illu[1] = texelFetch(u_tex8, p + ivec3(1, 0, 0) + param_step * i, 0).rgb;
		illu[2] = texelFetch(u_tex8, p + ivec3(0, 1, 0) + param_step * i, 0).rgb;
		illu[3] = texelFetch(u_tex8, p + ivec3(1, 1, 0) + param_step * i, 0).rgb;
		illu[4] = texelFetch(u_tex8, p + ivec3(0, 0, 1) + param_step * i, 0).rgb;
		illu[5] = texelFetch(u_tex8, p + ivec3(1, 0, 1) + param_step * i, 0).rgb;
		illu[6] = texelFetch(u_tex8, p + ivec3(0, 1, 1) + param_step * i, 0).rgb;
		illu[7] = texelFetch(u_tex8, p + ivec3(1, 1, 1) + param_step * i, 0).rgb;
		sh_illumination[i] = trilinear(w[0], w[1], w[2], 
		illu[0], illu[1], illu[2], illu[3],
		illu[4], illu[5], illu[6], illu[7]);

		float depth[8];
		depth[0] = texelFetch(u_tex9, p + ivec3(0, 0, 0) + param_step * i, 0).r;
		depth[1] = texelFetch(u_tex9, p + ivec3(1, 0, 0) + param_step * i, 0).r;
		depth[2] = texelFetch(u_tex9, p + ivec3(0, 1, 0) + param_step * i, 0).r;
		depth[3] = texelFetch(u_tex9, p + ivec3(1, 1, 0) + param_step * i, 0).r;
		depth[4] = texelFetch(u_tex9, p + ivec3(0, 0, 1) + param_step * i, 0).r;
		depth[5] = texelFetch(u_tex9, p + ivec3(1, 0, 1) + param_step * i, 0).r;
		depth[6] = texelFetch(u_tex9, p + ivec3(0, 1, 1) + param_step * i, 0).r;
		depth[7] = texelFetch(u_tex9, p + ivec3(1, 1, 1) + param_step * i, 0).r;
		sh_depth[i] = trilinear_1(w[0], w[1], w[2], 
		depth[0], depth[1], depth[2], depth[3],
		depth[4], depth[5], depth[6], depth[7]);
	}
	
	mat3 table;
    for(int i = 0; i < 3; i++)
        for(int j = 0; j < 3; j++)
            table[i][j] = N[i] * N[j];

	illumination = spherical_harmonics(sh_illumination, N, table);
	illumination *= 0.1;
	depth = spherical_harmonics_1(sh_depth, N, table);
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

	if(lighting == 1)
	{
		vec3 illumination;
		float depth;

		irradiance_volume(wpos, wnormal, illumination, depth);
		illumination *= 10.0;
		io_color = vec4(illumination, 1.0);
	}
	else if (lighting == 4) {
		io_color = vec4(bc, 1.0);
	} else discard;
}



