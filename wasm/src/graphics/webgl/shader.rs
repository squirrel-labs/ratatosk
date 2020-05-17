const VERTEX_SHADER: &str = include_str!("shader/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("shader/fragment.glsl");

enum ShaderType {
    Vertex,
    Fragment,
}
