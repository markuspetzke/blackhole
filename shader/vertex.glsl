#version 330 core
layout (location = 0) in vec3 aPos;
uniform mat4 transform;

out vec4 fragmentPos;
void main() {

	fragmentPos = vec4(aPos, 1.0);
  gl_Position = transform * fragmentPos;
}
