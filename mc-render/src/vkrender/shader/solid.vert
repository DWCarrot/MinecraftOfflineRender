#version 450

layout(location = 0) in vec3 geo;
layout(location = 1) in vec2 tex;
layout(location = 2) in vec4 color;

layout(location = 0) out vec2 v_tex;
layout(location = 1) out vec4 v_color;

layout(push_constant) uniform World { mat4 proj; } world;

layout(set = 0, binding = 0) uniform Position { vec3 pos; } position;
layout(set = 0, binding = 1) uniform Rotation { vec3 center; mat3 rotate; } rotation;

void main() {
    
    v_tex = tex;
    v_color = color;

    gl_Position = world.proj * vec4(rotation.rotate * (geo - rotation.center) + rotation.center, 1.0);
}