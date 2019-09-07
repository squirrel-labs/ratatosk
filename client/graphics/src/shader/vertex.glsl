#version 300 es

precision mediump float;

layout(location = 0) in vec2 position;

uniform mat3 transformation;
out vec2 tex_pos;

void main() {
    gl_Position = vec4((vec3(position, 1.0) * transformation).xy * vec2(9.0/16.0, 1.0), 0.0, 1.0);
    tex_pos = (-position + vec2(1.0, 1.0)) * 0.5;
}
