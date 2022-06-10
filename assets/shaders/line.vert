#version 330 core 

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 normal;
uniform mat4 projection;

void main() {
  vec2 p = position.xy + vec2(normal / 2.0);
  gl_Position = projection * vec4(p, 0.0, 1.0);
}
