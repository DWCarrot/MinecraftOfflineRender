#version 330

in vec4 v_color;
in vec3 v_tex;
in vec2 v_light;

uniform sampler2DArray textures;
uniform sampler2D light_map;

out vec4 fragColor;

void main() {
    fragColor = texture(textures, v_tex) * v_color * texture(light_map, v_light);
}