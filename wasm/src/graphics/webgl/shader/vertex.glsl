#version 300 es

precision mediump float;

in vec2 pos;
in mat3 mat;

in vec4 texture_bounds;
in uint texture_layer;

flat out vec4 frag_tex_bounds;
flat out uint frag_tex_layer;

out vec2 tex_pos;

void main() {
    gl_Position = vec4((vec3(pos, 1.0) * mat).xy, 0.0, 1.0);
    tex_pos = 0.5 * (vec2(1.0, 1.0) - pos);
    frag_tex_bounds = texture_bounds;
    frag_tex_layer = texture_layer;
}
