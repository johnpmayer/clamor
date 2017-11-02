#version 150 core
in vec4 a_pos;
//in ivec2 a_tex_coord;
//out vec2 v_TexCoord;
out vec4 v_pos_copy;
uniform mat4 u_model_view_proj;
void main() {
    //v_TexCoord = a_tex_coord;
    v_pos_copy = a_pos;
    gl_Position = u_model_view_proj * a_pos;
}
