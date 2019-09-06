#version 300 es

precision mediump float;

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 offset;

uniform mat3 transformation;
out vec2 tex_pos;

vec2 pixels = vec2(100.0, 100.0);

void main() {
    vec2 out_pos = (vec3(position, 1.0) * transformation).xy * vec2(9.0/16.0, 1.0) + offset;
    tex_pos = (-position + vec2(1.0, 1.0)) * 0.5;
    gl_Position = vec4(out_pos, 0.0, 1.0);
}
