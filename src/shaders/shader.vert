#version 450

const vec2 verts[3] = vec2[3](
    vec2(0.5f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f)
);

layout(location = 0) out vec2 vert;

void main() {
    vert = verts[gl_VertexIndex];
    gl_Position = vec4(vert - 0.5, 0.0, 1.0);
}
