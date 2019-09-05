struct vs_input_t
{
	float3 pos: POSITION;
	float2 uv: TEXCOORD;
};
struct vs_output_t
{
	float4 pos: SV_POSITION;
	float2 uv: TEXCOORD;
};

Texture2D tex: register(t0);
SamplerState samp: register(s0);

vs_output_t vs_main(vs_input_t input)
{
	vs_output_t output;
	output.pos = float4(input.pos, 1.0f);
	output.uv = input.uv;
	return output;
}

float4 ps_main(vs_output_t vs): SV_TARGET
{
	return tex.Sample(samp, vs.uv);
}
