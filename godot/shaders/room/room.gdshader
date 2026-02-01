shader_type spatial;

render_mode unshaded;
render_mode skip_vertex_transform;

const float ZERO_WIDTH = 2.667;

void vertex() {
	vec2 uv = UV * 2.0 - 1.0;

	VERTEX.x += ZERO_WIDTH * uv.x;
	VERTEX.z += ZERO_WIDTH * uv.y;

	vec4 vertex = MODELVIEW_MATRIX * vec4(VERTEX, 1.0);

	POSITION = (PROJECTION_MATRIX * (vertex));

	POSITION.x -= 0.01 * (VIEWPORT_SIZE.y / VIEWPORT_SIZE.x) * uv.x;
	POSITION.y -= 0.01 * uv.y;
}

void fragment() {
	ALBEDO = vec3(0.6, 0.6, 0.6);
}
