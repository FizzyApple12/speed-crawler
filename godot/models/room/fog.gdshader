shader_type spatial;

render_mode unshaded;
render_mode skip_vertex_transform;

const vec3 SEEN_COLOUR = vec3(0.2);
const vec3 HIDDEN_COLOUR = vec3(0.01);

void vertex() {
	vec2 uv = UV * 2.0 - 1.0;

	vec4 vertex = MODELVIEW_MATRIX * vec4(VERTEX, 1.0);

	POSITION = (PROJECTION_MATRIX * (vertex));

	POSITION.x -= 0.01 * (VIEWPORT_SIZE.y / VIEWPORT_SIZE.x) * uv.x;
	POSITION.y -= 0.01 * uv.y;
}

void fragment() {
	ALBEDO = HIDDEN_COLOUR;
}
