
use std::sync::Arc;

use web_sys::console;
use web_sys::WebGlRenderingContext;
use wasm_bindgen_futures::JsFuture;
use web_sys::WebGlProgram;


use crate::error::GfxError;
use crate::memory::Memory;
use crate::shader::Shader;
use crate::types::MAX_CHAR;

pub struct Uniform {
    name        : String,
    var_type    : u32,
    location    : i32,
    constant    : u8,
}

pub struct VertexAttrib {
    name        : String,
    var_type    : u32,
    location    : i32,
}

pub type DrawCallback        = dyn Fn();
pub type BindAttribCallback  = dyn Fn();

pub struct Program {
    name                : String,
    vertex_shader       : Shader,
    fragment_shader     : Shader,
    pid                 : Option<WebGlProgram>,
    uniform_array       : Vec<Uniform>,
    vertex_attrib_array : Vec<VertexAttrib>,
    draw_callback       : Option<Box<DrawCallback>>,
    bind_attr_callback  : Option<Box<BindAttribCallback>>,
    context             : Arc<WebGlRenderingContext>,
}

impl Program {
    
    pub async fn new(name                 : String,
                     vertex_shader_url    : String,
                     fragment_shader_url  : String,
                     relative_path        : bool,
                     debug_shader         : bool,
                     bind_attr_callback   : Option<Box<BindAttribCallback>>,
                     draw_callback        : Option<Box<DrawCallback>>,
                     context              : Arc<WebGlRenderingContext>,
                    ) -> Result<Self, GfxError>
    {
        use WebGlRenderingContext as Ctx;
        
        let memory = Memory::mopen(&vertex_shader_url, relative_path).await?;
        let mut vert_shader = Shader::new(&vertex_shader_url, 
                                          Ctx::VERTEX_SHADER,
                                          context.clone());
        vert_shader.compile(memory.as_str(), false);
        
        let memory = Memory::mopen(&fragment_shader_url, relative_path).await?;
        let mut frag_shader = Shader::new(&fragment_shader_url,
                                          Ctx::FRAGMENT_SHADER,
                                          context.clone());
        frag_shader.compile(memory.as_str(), false);
        Ok( Program {
                name,
                vertex_shader       : vert_shader,
                fragment_shader     : frag_shader,
                pid                 : None,
                uniform_array       : vec![],
                vertex_attrib_array : vec![],
                draw_callback,
                bind_attr_callback,
                context,
            } )
    }
    #[inline]
    pub fn pid(&self) -> &WebGlProgram {
        self.pid.as_ref().expect("Program pid not set")
    }
    fn delete_id(&mut self) {
        if self.pid.is_some() {
            self.context.delete_program(self.pid.as_ref());
            self.pid = None;
        }
    }
    fn add_vertex_attr(&mut self, name: String, var_type: u32) {
        let location = self.context.get_attrib_location(self.pid(), &name);
        self.vertex_attrib_array.push(
            VertexAttrib { 
                name,
                var_type,
                location
            });
    }
    fn add_uniform(&mut self, name: String, var_type: u32) {
        // TODO - I may need to store loc as-is in the Program struct if the
        //        code below doesn't work for i32. What is 'constant' for?
        let loc = self.context.get_uniform_location(self.pid(), &name).unwrap();
        self.uniform_array.push(
            Uniform {
                name,
                var_type,
                location: loc.as_f64().unwrap() as i32,
                constant: 0
            }
        )
    }
    pub fn link(&mut self, debug: bool) -> bool {
        use WebGlRenderingContext as Ctx;
        if self.pid.is_some() {
            return false;
        }
        
        // Create the program.
        self.pid = self.context.create_program();
        
        // Attach the shaders.
        self.context.attach_shader(self.pid(), self.vertex_shader.sid());
        self.context.attach_shader(self.pid(), self.fragment_shader.sid());
        
        // Invoke the binding done callback.
        if let Some(callback) = &self.bind_attr_callback {
            callback();
        }        
        // Link the program.
        self.context.link_program(self.pid());
        
        // If debug is true, print out any diagnostic info that just happened.
        if debug {
            if let Some(log) = self.context.get_program_info_log(self.pid()) {
                if !log.is_empty() {
                    let msg = format!("[ {} ]\n", self.name);
                    console::log_2(&msg.into(), &log.into());
                }
            }
        }
        // Check the link status and exit with 'false' if there was a failure.
        let status = self.context.get_program_parameter(self.pid(), 
                                                        Ctx::LINK_STATUS);
        if status.is_falsy() {
            self.delete_id();
            return false;
        }
        // Get the number of attributes and add them to the attribute array.
        let nattr = self.context.get_program_parameter(self.pid(), 
                                                       Ctx::ACTIVE_ATTRIBUTES);
        let nattr = nattr.as_f64().unwrap() as u32;
        
        for i in 0..nattr {
            let attrib = self.context.get_active_attrib(self.pid(), i).unwrap();
            self.add_vertex_attr(attrib.name(), attrib.type_());
        }
        // Get the number of uniforms and add them to the uniform array.
        let nuni = self.context.get_program_parameter(self.pid(),
                                                      Ctx::ACTIVE_UNIFORMS);
        let nuni = nuni.as_f64().unwrap() as u32;
        
        for i in 0..nuni {
            let uni = self.context.get_active_uniform(self.pid(), i).unwrap();
            self.add_uniform(uni.name(), uni.type_());
        }
        
        true
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.delete_id();
    }
}







