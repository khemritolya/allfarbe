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

vec2 wrap_index(float a, float b) {
    return vec2(fmod(a, 1.0), fmod(b, 1.0));
}

float smooth_sample(sampler2D tex, vec2 coords) {
    float count = 0;
    float result = 0;
    for (float i = -0.005; i <= 0.0051; i += 0.005) {
        for (float j = -0.005; j <= 0.0051; j += 0.005) {
            count++;
            vec2 index = wrap_index(coords.x + i, coords.y + j);
            result += layer(texture(tex, index));
        }
    }

    return result / count;
}

void main() {
    vec2 position = data.xy;
    float time = data.z;

    float layered_noise = layer(texture(tex, texture_coords));
    float smooth_noise = smooth_sample(tex, texture_coords);
    float warp = fmod(smooth_noise + time / 60000.0 + layered_noise / 10, 1.0);

    vec2 index = wrap_index(texture_coords.x + warp, texture_coords.y + warp);
    float value = smooth_sample(tex, index) * 0.65 + layered_noise * 0.35;
    color = vec4(sqrt(value) + 0.1, value * value * 1.5, 0.0, 1.0);
}