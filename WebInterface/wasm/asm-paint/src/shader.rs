use web_sys::{WebGlProgram, WebGl2RenderingContext};

const VERTEX_SHADER: &str =
r#"#version 300 es
in vec4 pos;
void main() {
    gl_Position = pos;
}
"#;

const FRAGMENT_SHADER: &str =
r#"#version 300 es
precision mediump float;
out vec4 color;

void main() {
    color = vec4(1, 0, 0, 1);
}
"#;

pub struct Shaders {
    program: Option<WebGlProgram>,
    pos_loc: i32,
}

impl Shaders {
    pub fn new() -> Self {
        Self {
            program: None,
            pos_loc: -1
        }
    }

    fn create_program(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
        self.program = Some(ctx.create_program().ok_or("could not create program id")?);
        Ok(())
    }

    fn create_shader(&mut self, ctx: &WebGl2RenderingContext,
                     shader_type: u32, source: &str) -> Result<(), String> {
        let program = self.program.as_ref().ok_or("could not find created program")?;
        let shader = ctx.create_shader(shader_type)
            .ok_or("could not create shader")?;
        ctx.shader_source(&shader, source);
        ctx.compile_shader(&shader);
        let status = ctx.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS);
        if status == wasm_bindgen::JsValue::TRUE {
            ctx.attach_shader(program, &shader);
            Ok(())
        } else {
            Err(format!("\n{}", ctx.get_shader_info_log(&shader).unwrap_or_default()))
        }
    }

    fn create_vertex_shader(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
        self.create_shader(ctx, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER)
    }

    fn create_fragment_shader(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
        self.create_shader(ctx, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER)
    }

    fn compile(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
        let program = self.program.as_ref().ok_or("could not find created program")?;
        ctx.link_program(program);
        let status = ctx.get_program_parameter(program, WebGl2RenderingContext::LINK_STATUS);
        if status == wasm_bindgen::JsValue::TRUE {
            Ok(())
        } else {
            Err(format!("\n{}", ctx.get_program_info_log(program).unwrap_or_default()))
        }
    }

    pub fn init(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
        debug!("create program");
        self.create_program(ctx)
            .map_err(|e| { error!("webgl2 create program: {}", e); e})?;
        debug!("create vertex shader");
        self.create_vertex_shader(ctx)
            .map_err(|e| { error!("webgl2 create vertex shader: {}", e); e})?;
        debug!("create fragment shader");
        self.create_fragment_shader(ctx)
            .map_err(|e| { error!("webgl2 create fragment shader: {}", e); e})?;
        debug!("compile shader program");
        self.compile(ctx)
            .map_err(|e| { error!("webgl2 shader: {}", e); e})?;
        let program = self.program.as_ref().ok_or("could not find created program")?;
        self.pos_loc = ctx.get_attrib_location(program, "pos");
        trace!("got attrib location 'pos'({})", self.pos_loc);
        info!("initialised shader program");
        Ok(())
    }

    pub fn remove(&mut self, ctx: &WebGl2RenderingContext) {
        ctx.delete_program(self.program.as_ref())
    }
}
