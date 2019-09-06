#version 300 es

precision mediump float;

out vec4 color;
in vec2 tex_pos;

uniform mediump sampler2D g_texture;

void main() {
    //color = vec4(0.0, 1.0, 0.0, 1.0);
    //color = texture2D(g_texture, tex_pos);
    color = texture(g_texture, tex_pos);
}
