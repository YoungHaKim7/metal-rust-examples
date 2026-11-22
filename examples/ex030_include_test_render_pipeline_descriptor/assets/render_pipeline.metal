#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut triangle_vertex(uint id [[vertex_id]]) {
    float2 corners[3] = {float2(-0.6, -0.6), float2(0.6, -0.6), float2(0.0, 0.6)};
    return VertexOut{float4(corners[id], 0.0, 1.0), float4(0.2, 0.5, 1.0, 1.0)};
}

fragment float4 triangle_fragment(VertexOut in [[stage_in]]) {
    return in.color;
}
