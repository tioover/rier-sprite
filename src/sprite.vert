#version 330 core
uniform mat4 camera;
uniform mat4 transform;
uniform sampler2D tex;
in vec2 position;
in vec2 tex_coords;
out vec2 uv;

void main() {
    vec2 size = vec2(textureSize(tex, 0));
    uv = tex_coords / size;
    gl_Position = camera * transform * vec4(position, 0.0, 1.0);
}
