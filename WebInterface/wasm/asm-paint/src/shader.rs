use web_sys::{WebGlProgram, WebGl2RenderingContext};

const VERTEX_SHADER: &str = r#"#version 300 es
in vec4 pos;
void main() {
    gl_Position = pos;
}
"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es
precision mediump float;
out vec4 color;

void main() {
    color = vec4(1, 0, 0, 1);
}
"#;

pub struct Shaders {
    program: Option<WebGlProgram>,
}

impl Shaders {
    pub fn new() -> Self {
        Self {
            program: None,
        }
    }

    pub fn create_program(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
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

    pub fn create_vertex_shader(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
        self.create_shader(ctx, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER)
    }

    pub fn create_fragment_shader(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
        self.create_shader(ctx, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER)
    }

    pub fn compile(&mut self, ctx: &WebGl2RenderingContext) -> Result<(), String> {
        let program = self.program.as_ref().ok_or("could not find created program")?;
        ctx.link_program(program);
        let status = ctx.get_program_parameter(program, WebGl2RenderingContext::LINK_STATUS);
        if status == wasm_bindgen::JsValue::TRUE {
            Ok(())
        } else {
            Err(format!("\n{}", ctx.get_program_info_log(program).unwrap_or_default()))
        }
    }

    pub fn remove(&mut self, ctx: &WebGl2RenderingContext) {
        ctx.delete_program(self.program.as_ref())
    }
}
