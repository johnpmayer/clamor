#version 150 core

in vec4 v_pos_copy;
in float v_color_copy;
out vec4 o_Color;

void main() {
    o_Color = v_pos_copy * v_color_copy;
}
