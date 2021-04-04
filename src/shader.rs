
use std::sync::Arc;

use web_sys::console;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlShader;

pub struct Shader {
    name    : String,
    sh_type : u32,
    sid     : Option<WebGlShader>,
    context : Arc<WebGlRenderingContext>,
}

impl Shader {
    pub fn new(name     : &str, 
               sh_type  : u32, 
               context  : Arc<WebGlRenderingContext>
              ) -> Self 
    {
        Shader {
            name : name.to_string(),
            sid  : None,
            sh_type,
            context,
        }
    }
    fn delete_id(&mut self) {
        if self.sid.is_some() {
            self.context.delete_shader(self.sid.as_ref());
            self.sid = None;
        }
    }
    pub fn compile(&mut self, code: &str, debug: bool) -> bool {
        use web_sys::WebGlRenderingContext as Ctx;
        if self.sid.is_some() {
            return false;
        } 
        // Create shader.
        self.sid = self.context.create_shader(self.sh_type);
        let sid  = self.sid.as_ref().unwrap();
        let ctx  = &self.context;        

        // Set source and compile.
        ctx.shader_source(sid, code);
        ctx.compile_shader(sid);
        
        // If debug is true, print out any diagnostic info that just happened.
        if debug {
            if let Some(log) = ctx.get_shader_info_log(sid) {
                if !log.is_empty() {
                    let typ = if self.sh_type == Ctx::VERTEX_SHADER 
                                   { "GL_VERTEX_SHADER"   } 
                              else { "GL_FRAGMENT_SHADER" };

                    let msg = format!("[ {}: {} ]\n", self.name, typ);
                    
                    console::log_2(&msg.into(), &log.into());
                }
            }
        }
        // Check to make sure the compilation was successful.
        let status = ctx.get_shader_parameter(sid, Ctx::COMPILE_STATUS);
        if status.is_falsy() {
            self.delete_id();
            false
        } else {
            true
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.delete_id();
    }
}
