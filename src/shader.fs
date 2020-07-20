#version 140
in vec2 texture_coords;
in vec3 data;

out vec4 color;

uniform sampler2D tex;
float fmod(float x, float y) {
  return x - y * floor(x/y);
}
float layer(vec4 v) {
    return v[3] / 2.0 + v[2] / 4.0 + v[1] / 8.0 + v[0] / 16.0;
}

void main() {
    vec2 position = data.xy;
    float time = data.z;

    vec4 noise = texture(tex, texture_coords);
    float warp = fmod(layer(noise) + time / 40000.0, 1.0);

    vec2 index = vec2(fmod(texture_coords.x + warp, 1.0), fmod(texture_coords.y + warp, 1.0));
    vec4 warped = texture(tex, index);
    float value = layer(warped);
    color = vec4(value, value * value, 0.0, 1.0);
}