#version 150 core
in vec4 a_pos;
in float a_color;
out float v_color_copy;
uniform mat4 u_model_view_proj;
void main() {
    //v_TexCoord = a_tex_coord;
    v_color_copy = a_color;
    gl_Position = u_model_view_proj * a_pos;
}
