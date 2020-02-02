#version 450

layout(location = 0) in vec3 geo;
layout(location = 1) in vec2 tex;
layout(location = 2) in vec4 color;

layout(location = 0) out vec2 v_tex;
layout(location = 1) out vec4 v_color;
layout(location = 2) out vec3 v_normal;
layout(location = 3) out vec2 v_lit_coord;

layout(set = 0, binding = 0) uniform World { mat4 proj; } world;
layout(set = 0, binding = 1) uniform Position { vec3 pos; } position;
layout(set = 0, binding = 2) uniform Rotation { vec3 center; mat3 rotate; } rotation;
layout(set = 0, binding = 3) uniform Light { vec3 normal; vec2 lit_coord; } light;

void main() {
    
    v_tex = tex;
    v_color = color;
    v_normal = light.normal;
    v_lit_coord = light.lit_coord;

    gl_Position = world.proj * vec4(rotation.rotate * (geo - rotation.center) + rotation.center, 1.0);
}