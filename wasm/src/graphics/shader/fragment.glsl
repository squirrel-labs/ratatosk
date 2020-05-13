#version 300 es

precision mediump float;

out vec4 color;
in vec2 tex_pos;

uniform mediump sampler2D g_texture;

void main() {
    vec2 tp = tex_pos;
    tp.y = 1.0 - tp.y;
    color = texture(g_texture, tp);
    if (gl_FragCoord.y > 500.0) {
        color = vec4(vec3(length(color) * 0.4), 1.0);
    }
    if (color.a == 0.0) discard;
}
