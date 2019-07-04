#version 300 es

precision highp float;

in vec3 outcolor;
out vec4 color;

void main() {
    color = vec4(0.0, 1.0, 1.0, 1.0);
    color = vec4(outcolor, 1.0);
}
