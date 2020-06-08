#version 300 es

precision mediump float;

uniform mediump sampler2DArray g_texture;

flat in vec4 frag_tex_bounds;
flat in uint frag_tex_layer;

in vec2 tex_pos;

out vec4 color;

void main() {
    color = texture(g_texture, vec3(frag_tex_bounds.xy + frag_tex_bounds.zw * vec2(tex_pos.x, 1.0 - tex_pos.y), float(frag_tex_layer)));
    if (color.a == 0.0) discard;
}
