#version 330 core
in vec3 wnormal;
in vec3 wpos;
in vec2 uv;
layout (location = 2) out vec3 io_basecolor;
layout (location = 7) out int io_lighting;

uniform vec3 u_sh_coeff[18];

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

void main()
{
    vec3 p = normalize(wnormal); 

    mat3 table;
    for(int i = 0; i < 3; i++)
        for(int j = 0; j < 3; j++)
            table[i][j] = p[i] * p[j];

    vec3 color = spherical_harmonics(u_sh_coeff, p, table);
    io_basecolor = color;
    io_lighting = 4;
}
