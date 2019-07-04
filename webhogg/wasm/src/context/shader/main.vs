#version 300 es

precision highp float;

layout(location=0) in vec2 position;

uniform vec2 offset;
uniform mat3 transform;

out vec3 outcolor;

void main() {
    gl_Position = vec4((transform * vec3(position, 1.0)).xy + offset, 0.0, 1.0);
    outcolor = transform[2];
}
