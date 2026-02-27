#version 330 core
out vec4 FragColor;

uniform vec4 objectColor;
uniform vec2 lightPos;
uniform vec2 ballPos;

in vec4 fragmentPos;
float calc_basic_light() {

	vec2 worldPos = fragmentPos.xy + ballPos;

	float dist = length(worldPos - lightPos);
	return 1.0 - clamp(dist / 800.0, 0.0, 1.0);

}
void main() {

	float light = calc_basic_light();
  FragColor = vec4(objectColor.rgb * light, objectColor.a );
}
