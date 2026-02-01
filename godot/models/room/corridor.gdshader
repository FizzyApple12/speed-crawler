shader_type spatial;

render_mode unshaded;

instance uniform float seen : hint_range(0.0, 1.0);

const vec3 SEEN_COLOUR = vec3(0.2);
const vec3 HIDDEN_COLOUR = vec3(0.01);

void fragment() {
	ALBEDO = mix(HIDDEN_COLOUR, SEEN_COLOUR, seen);
}
