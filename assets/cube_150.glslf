#version 150 core
//in vec2 v_TexCoord;
out vec4 o_Color;
//uniform sampler2D t_color;
in vec4 v_pos_copy;
in float v_color_copy;

/*
vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}
*/

void main() {
    //vec4 tex = texture(t_color, v_TexCoord);
    //float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    //o_Color = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    //o_Color = vec4(1.0,0.0,0.0,1.0);

    // get angle from x & y, convert to hue
    // convert ((z + 1) / 2) to saturation and value
    
    // o_Color = vec4(v_color_copy,v_color_copy,v_color_copy,1.0);
    o_Color = v_pos_copy * v_color_copy;
}
