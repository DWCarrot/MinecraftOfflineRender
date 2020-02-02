#version 450

layout(location = 0) in vec2 v_tex;
layout(location = 1) in vec4 v_color;
layout(location = 2) in vec3 v_normal;
layout(location = 3) in vec2 v_lit_coord;

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 1) uniform Position { vec3 pos; } position;
layout(set = 0, binding = 2) uniform Rotation { vec3 center; mat3 rotate; } rotation;
layout(set = 0, binding = 4) uniform sampler2D tex_sampler;
layout(set = 0, binding = 5) uniform sampler2D lit_map; // artificial-light x sky-light

void main() {

    f_color = v_color * texture(tex_sampler, v_tex) * texture(lit_map, v_lit_coord);
}