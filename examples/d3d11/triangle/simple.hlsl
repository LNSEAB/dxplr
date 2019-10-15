struct vs_input_t
{
	float3 position: POSITION;
	float4 color: COLOR;
};

struct vs_output_t
{
	float4 position: SV_POSITION;
	float4 color: COLOR;
};

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
