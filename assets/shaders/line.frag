#version 330 core

uniform vec3 diffuse;
uniform float opacity;

void main() {
  gl_FragColor = vec4(diffuse, opacity);
}
