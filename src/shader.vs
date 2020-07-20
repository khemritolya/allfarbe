#version 140
in vec2 position;
in vec2 tex_coords;

out vec2 texture_coords;
out vec3 data;

uniform mat4 matrix;
void main() {
    texture_coords = tex_coords;
    data = vec3(position, matrix[0][0]);
    gl_Position = vec4(position, 0.0, 1.0);
}