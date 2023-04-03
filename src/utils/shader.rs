pub mod vertex
{
    vulkano_shaders::shader!{
        ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}"
    }
}

pub mod fragment
{
    vulkano_shaders::shader!{
        ty: "fragment",
        src: "
#version 450

#extension GL_ARB_gpu_shader_fp64 : enable
#extension GL_ARB_gpu_shader5 : enable

#pragma optionNV(fastmath off)
#pragma optionNV(fastprecision off)

layout(set=0, binding=0) uniform Center { vec2 center; };
layout(set=0, binding=1) uniform Offset { vec2 offset; };
layout(set=0, binding=2) uniform Zoom { float zoom; };
layout(location=0) out vec4 f_color;

void main()
{
    vec2 norm_coordinates = vec2(gl_FragCoord.x - center.x, gl_FragCoord.y - center.y) / (zoom * 1000.0) - offset / 1000.0;
    vec2 z = vec2(0.0, 0.0);
    // vec2 z = norm_coordinates;

    dvec2 c = norm_coordinates * 2.0 - vec2(1.0, 0.0);
    // vec2 c = vec2(-0.8, 0.156);

    float i;
    for(i = 0.0; i < 1.0; i += 0.005)
    {
        z = vec2(
            z.x * z.x - z.y * z.y + c.x,
            2.0 * z.x * z.y + c.y
        );

        if (length(z) > 4.0)
        {
            break;
        }
    }

    f_color = vec4(vec3(i), 1.0);
}"
    }
}
