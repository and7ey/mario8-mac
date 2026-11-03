#version 450
layout(set = 0, binding = 0, std430) readonly buffer ResBuf { uint flags; } res;
layout(set = 0, binding = 1) uniform texture2D src_tex;
layout(set = 1, binding = 0) uniform sampler src_smp;
layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 color;
void main() {
    vec4 c = texture(sampler2D(src_tex, src_smp), uv);
    float k = 1.0 - 0.25 * float(res.flags & 1u);
    color = vec4(c.rgb * k, 1.0);
}
