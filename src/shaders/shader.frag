#version 450

layout(location = 0) in vec2 vert;
layout(location = 0) out vec4 color;

void main() {
    color = vec4(vert, 0.5, 1.0);
}
