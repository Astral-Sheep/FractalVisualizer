pub mod vertex
{
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "src/glsl/vertex.glsl"
    }
}

pub mod fragment
{
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "src/glsl/fragment.glsl"
    }
}
