use web_sys::WebGl2RenderingContext as GlContext;

pub struct Color4(f32, f32, f32, f32);

impl Color4 {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color4 {
        Color4(r, g, b, a)
    }
}

pub struct WebGl2 {
    gl: GlContext,
}

impl WebGl2 {
    pub fn from_context(context: GlContext) -> Self {
        WebGl2 {
            gl: context,
        }
    }

    pub fn clear(&self, color: Color4) {
        self.gl.clear_color(color.0, color.1, color.2, color.3);
        self.gl.clear(GlContext::COLOR_BUFFER_BIT);
    }
}
