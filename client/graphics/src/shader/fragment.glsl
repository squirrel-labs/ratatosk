#version 300 es

precision mediump float;

out vec4 color;
in vec2 tex_pos;

uniform mediump sampler2D g_texture;

void main() {
    vec2 tp = tex_pos;
    // tp.y = 1.0 - tp.y;
    color = texture(g_texture, tp);
    if (color.a == 0.0) discard;
}
