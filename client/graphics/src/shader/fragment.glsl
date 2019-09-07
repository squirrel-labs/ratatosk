#version 300 es

precision mediump float;

out vec4 color;
in vec2 tex_pos;

uniform mediump sampler2D g_texture;

void main() {
    color = texture(g_texture, tex_pos);
    if (color.a == 0.0) discard;
}
