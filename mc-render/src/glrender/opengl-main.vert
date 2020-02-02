#version 330

in ivec3 loc;
in vec3 pos;
in vec2 tex;
in int tex_id;
in uint color;
in uint light;

uniform mat4 world;
uniform ivec3 center;

out vec4 v_color;
out vec3 v_tex;
out vec2 v_light;

void main() {
    v_color = vec4((color << 24) >> 24, (color << 16) >> 24, (color << 8) >> 24, (color << 0) >> 24) / 255.0;
    v_tex = vec3(tex / 16.0, tex_id);
    v_light = vec2((light << 28) >> 28, (light << 24) >> 28) / 16.0;
    vec3 position = pos / 16.0 + vec3(loc - center);
    gl_Position =  world * vec4(position, 1.0);
}
