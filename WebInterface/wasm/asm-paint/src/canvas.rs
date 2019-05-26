use web_sys;
use web_sys::{WebGl2RenderingContext};
use wasm_bindgen::JsCast;
use crate::shader::Shaders;

pub struct Canvas {
    element: web_sys::HtmlCanvasElement,
    ctx: WebGl2RenderingContext,
    shaders: Shaders,
}

impl Canvas {
    pub fn new(element: web_sys::Element) -> Option<Self> {
        let element: web_sys::HtmlCanvasElement =
            element.dyn_into::<web_sys::HtmlCanvasElement>()
            .ok()?;
        debug!("create webgl2 context");
        let ctx = element.get_context("webgl2").ok()??
            .dyn_into::<WebGl2RenderingContext>().ok()?;
        info!("created webgl2 context successfully");
        Some(Self {
            element, ctx,
            shaders: Shaders::new(),
        })
    }

    pub fn init(&mut self) -> Result<(), ()> {
        debug!("create program");
        self.shaders.create_program(&self.ctx)
            .map_err(|e| error!("webgl2 create program: {}", e))?;
        debug!("create vertex shader");
        self.shaders.create_vertex_shader(&self.ctx)
            .map_err(|e| error!("webgl2 create vertex shader: {}", e))?;
        debug!("create fragment shader");
        self.shaders.create_fragment_shader(&self.ctx)
            .map_err(|e| error!("webgl2 create fragment shader: {}", e))?;
        debug!("compile shader program");
        self.shaders.compile(&self.ctx)
            .map_err(|e| error!("webgl2 shader: {}", e))
    }
}
