#version 300 es

precision mediump float;

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position * 0.5, 0.0, 1.0);
}
