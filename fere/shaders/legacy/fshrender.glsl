#version 330 core
in vec3 rpos;
in vec3 normal;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform vec4 u_color;

layout (location = 0) out vec4 io_col;

struct SLightVolume
{
	mat4 rtrans; // room-space transformation 
    vec3 cellsize;
    ivec3 nums;
    sampler3D tex;
};
uniform SLightVolume u_lv;
uniform int u_t;

#define access(i,j,k) (i*4 + j*2 + k)

vec4 interpolate(int d)
{
    vec4 s[8];
    float f[3];
    ivec3 index;

    for(int i = 0; i < 3; i++)
    {
        index[i] = rpos[i] < 0 ? -1 : int(rpos[i] / u_lv.cellsize[i]);
        f[i] = 1 - (rpos[i] - index[i] * u_lv.cellsize[i])
        / u_lv.cellsize[i];
    }
    
    for(int i = 0; i < 2; i++)
        for(int j = 0; j < 2; j++)
            for(int k = 0; k < 2; k++)
            {
                ivec3 p = index + ivec3(i,j,k);
                p[0] = clamp(p[0], 0, u_lv.nums[0] - 1);
                p[1] = clamp(p[1], 0, u_lv.nums[1] - 1);
                p[2] = clamp(p[2], 0, u_lv.nums[2] - 1);
                p[2] += u_lv.nums[2] * d;
                s[access(i,j,k)] =
                texelFetch(u_lv.tex, p, 0); 
            }

    for(int i = 0; i < 2; i++)
        for(int j = 0; j < 2; j++)
            s[access(i,j,0)] =
            s[access(i,j,0)] * f[2] + s[access(i,j,1)] * (1-f[2]);

    for(int i = 0; i < 2; i++)
        s[access(i,0,0)] =
        s[access(i,0,0)] * f[1] + s[access(i,1,0)] * (1-f[1]);

    //return s[7];
    return s[access(0,0,0)] * f[0] + s[access(1,0,0)] * (1-f[0]);
}
void main()
{
    vec4 coeff[16];
    for(int i = 0; i < 16; i++)
        coeff[i] = interpolate(i);

    vec4 final = vec4(0);
    vec3 p = normalize(normal);
    
    mat3 table;
    for(int i = 0; i < 3; i++)
        for(int j = 0; j < 3; j++)
            table[i][j] = p[i] * p[j];

    for(int i = 0; i < 4; i++)
    {
        final[i] += coeff[0][i];

        final[i] += coeff[1][i] * p[1];
        final[i] += coeff[2][i] * p[2];
        final[i] += coeff[3][i] * p[0];

        final[i] += coeff[4][i] * table[0][1];
        final[i] += coeff[5][i] * table[1][2];
        final[i] += coeff[6][i] * (3 * table[2][2] - 1);
        final[i] += coeff[7][i] * table[0][2];
        final[i] += coeff[8][i] * (table[0][0] - table[1][1]);

        final[i] += coeff[9][i] * p[1] * (3 * table[0][0] - table[1][1]);
        final[i] += coeff[10][i] * p[2] * table[0][1];
        final[i] += coeff[11][i] * p[1] * (4 * table[2][2] - table[0][0] - table[1][1]);
        final[i] += coeff[12][i] * p[2] * (2 * table[2][2] - 3 * table[0][0] - 3 * table[1][1]);
        final[i] += coeff[13][i] * p[0] * (4 * table[2][2] - table[0][0] - table[1][1]);
        final[i] += coeff[14][i] * p[2] * (table[0][0] - table[1][1]);
        final[i] += coeff[15][i] * p[0] * (table[0][0] - 3 * table[1][1]);
    }
    final *= 0.17;
    final *= final;
    io_col = vec4(vec3(final.xyz * final[3]), 1);
}
