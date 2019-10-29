#include "local.hlsl"
#include <system.hlsl>

vs_output_t vs_main(vs_input_t input)
{
	vs_output_t output;
	output.position = float4(input.position, 1.0f);
	output.color = input.color;
	return output;
}

float4 ps_main(vs_output_t vs_data): SV_TARGET
{
	return vs_data.color;
}
