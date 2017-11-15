#version 150 core

uniform mat4 u_model_view_proj;
in vec4 a_pos;
in float a_color;
out vec4 v_pos_copy;
out float v_color_copy;

void main() {
    //v_TexCoord = a_tex_coord;
    v_pos_copy = a_pos;
    v_color_copy = a_color;
    gl_Position = u_model_view_proj * a_pos;
}
