#version 300 es

precision highp float;

layout(location=0) in vec2 position;

uniform vec2 offset;
uniform vec2 size;

void main() {
    gl_Position = vec4((position * size) + offset, 0.0, 1.0);
}
