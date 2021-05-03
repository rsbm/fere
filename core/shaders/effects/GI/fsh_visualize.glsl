#version 330 core
in vec3 wnormal;
in vec3 wpos;
in vec2 uv;
layout (location = 0) out vec4 io_color;

struct SLightVolume
{
	mat4 trans; // chamber->world transformation
	
	/// Chamber origin + offset' = probe of (1,1,1), where (0,0,0) is padded one
	/// offset' - chambercenter offset = offset;
	vec3 offset;

    vec3 cellsize;
    ivec3 nums; // with padding
	vec3 chamsize; // cellsize * nums, with paddin

	int params; // sh dimension
};
uniform SLightVolume u_lv;

uniform sampler3D u_tex0;

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
    result += sh_coeff[13] * (table[2][2] * (35 * table[2][2] - 30) + 3);
    result += sh_coeff[14] * table[0][2] * (7 * table[2][2] - 3);
    result += sh_coeff[15] * (table[0][0] - table[1][1]) * (7 * table[2][2] - 1);
    result += sh_coeff[16] * table[0][2] * (table[0][0] - 3 * table[1][1]);
    result += sh_coeff[17] * (table[0][0] * (table[0][0] - 3 * table[1][1]) - table[1][1] * (3 * table[0][0] - table[1][1]));

	return result;
}

void irradiance_volume(vec3 P, vec3 N, out vec3 diffuse, out vec3 illumination, out float depth)
{
	P += u_lv.cellsize - u_lv.offset;
	P /= u_lv.chamsize;
	P *= u_lv.nums;

	ivec3 p = ivec3(P); // lowest x,y,z probe
	vec3 w = P - vec3(p);

	vec3 sh_diffuse[18]; //max 18
	vec3 sh_illumination[18];
	float sh_depth[18];

	ivec3 param_step = ivec3(0, 0, u_lv.nums[2]);
	for(int i = 0; i < u_lv.params; i++)
	{
		sh_illumination[i] = 
        vec3(texelFetch(u_tex0, p + ivec3(0, 0, 0) + param_step * i, 0).r * 0.3, 0, 0);
	}
	
	mat3 table;
    for(int i = 0; i < 3; i++)
        for(int j = 0; j < 3; j++)
            table[i][j] = N[i] * N[j];

	illumination = spherical_harmonics(sh_illumination, N, table);
}


void main()
{
    vec3 p = normalize(wnormal); 

    vec3 diffuse;
	vec3 illumination;
	float depth;

    irradiance_volume(wpos, p, diffuse, illumination, depth);
    io_color = vec4(illumination, 1);
}
