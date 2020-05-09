#version 300 es

precision mediump float;

layout(location = 0) in vec2 position;

uniform mat3 transformation;
out vec2 tex_pos;

vec2 pixels = vec2(100.0, 100.0);

void main() {
    const vec2 aspect = vec2(1.0, 1.0); // vec2(9.0/16.0, 1.0)
    vec2 out_pos = (vec3(position, 1.0) * transformation).xy * aspect;
    tex_pos = 0.5 * (vec2(1.0, 1.0) - position);
    gl_Position = vec4(out_pos, 0.0, 1.0);
}
