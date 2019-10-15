RWStructuredBuffer<uint> buf;

[numthreads(1, 1, 1)]
void cs_main(uint3 tid: SV_DispatchThreadID)
{
	for(int i = 0; i < 10; ++i) {
		buf[i] = buf[i] + (i + 1);
	}
}
