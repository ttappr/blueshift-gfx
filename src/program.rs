
use std::sync::Arc;

use wasm_bindgen_futures::JsFuture;

use web_sys::console;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlUniformLocation;


use crate::error::GfxError;
use crate::memory::Memory;
use crate::shader::Shader;
use crate::types::MAX_CHAR;

pub struct Uniform {
    name        : String,
    var_type    : u32,
    //location    : i32,
    location    : WebGlUniformLocation,
    constant    : u8,
}

pub struct VertexAttrib {
    name        : String,
    var_type    : u32,
    location    : i32,
}

pub type DrawCallback     = dyn Fn(&Program);
pub type BindAttrCallback = dyn Fn();

pub struct Program {
    name                : String,
    vertex_shader       : Shader,
    fragment_shader     : Shader,
    pid                 : Option<WebGlProgram>,
    uniform_array       : Vec<Uniform>,
    vertex_attrib_array : Vec<VertexAttrib>,
    draw_callback       : Option<Box<DrawCallback>>,
    bind_attr_callback  : Option<Box<BindAttrCallback>>,
    context             : Arc<WebGlRenderingContext>,
}

impl Program {
    
    pub async fn new(name                 : String,
                     vertex_shader_url    : String,
                     fragment_shader_url  : String,
                     bind_attr_callback   : Option<Box<BindAttrCallback>>,
                     draw_callback        : Option<Box<DrawCallback>>,
                     context              : Arc<WebGlRenderingContext>,
                    ) -> Result<Self, GfxError>
    {
        use WebGlRenderingContext as Ctx;
        
        let memory = Memory::mopen(&vertex_shader_url).await?;
        let mut vert_shader = Shader::new(&vertex_shader_url, 
                                          Ctx::VERTEX_SHADER,
                                          context.clone());
        vert_shader.compile(memory.as_str());
        
        let memory = Memory::mopen(&fragment_shader_url).await?;
        let mut frag_shader = Shader::new(&fragment_shader_url,
                                          Ctx::FRAGMENT_SHADER,
                                          context.clone());
        frag_shader.compile(memory.as_str());
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
    pub fn set_draw_callback(&mut self, draw_callback: Box<DrawCallback>) {
        self.draw_callback = Some(draw_callback);
    }
    pub fn set_bind_attr_callback(&mut self, 
                                  bind_attr_callback: Box<BindAttrCallback>)
    {
        self.bind_attr_callback = Some(bind_attr_callback);
    }
    pub fn draw(&self) {
        self.context.use_program(self.pid.as_ref());
        if let Some(callback) = &self.draw_callback {
            callback(self);
        }
    }
    fn delete_id(&mut self) {
        if self.pid.is_some() {
            self.context.delete_program(self.pid.as_ref());
            self.pid = None;
        }
    }
    fn add_vertex_attr(&mut self, name: String, var_type: u32) {
        // TODO - Find out why we're even storing these. These can be retrieved
        //        from the context anyway.
        let location = self.context.get_attrib_location(self.pid(), &name);
        self.vertex_attrib_array.push(
            VertexAttrib { 
                name,
                var_type,
                location
            });
    }
    fn add_uniform(&mut self, name: String, var_type: u32) {
        // TODO - Find out why we're even storing these. These can be retrieved
        //        from the context anyway.
        // TODO - I may need to store loc as-is in the Program struct if the
        //        code below doesn't work for i32. What is 'constant' for?
        let location = self.context.get_uniform_location(self.pid(), &name)
                                   .unwrap();
        self.uniform_array.push(
            Uniform {
                name,
                var_type,
                location,
                constant: 0
            }
        )
    }
    fn get_vertex_attrib_location(&self, name: &str) -> i32 {
        let attr = self.vertex_attrib_array
                       .iter()
                       .find(|a| a.name == name)
                       .expect(&format!("{}.{} wasn't found.", 
                                        self.name, name));
        attr.location
    }
    fn get_uniform_location(&self, name: &str) -> &WebGlUniformLocation {
        let uni = self.uniform_array
                      .iter()
                      .find(|u| u.name == name)
                      .expect(&format!("{}.{} wasn't found.", 
                                       self.name, name));
        &uni.location
    }
    pub fn link(&mut self) -> bool {
        use WebGlRenderingContext as Ctx;
        if self.pid.is_some() {
            return false;
        }
        let ctx = &*self.context;
        
        // Create the program.
        self.pid = ctx.create_program();
        
        let ref pid = self.pid.as_ref().unwrap().clone();
        
        // Attach the shaders.
        ctx.attach_shader(pid, self.vertex_shader.sid());
        ctx.attach_shader(pid, self.fragment_shader.sid());
        
        // Invoke the binding done callback.
        if let Some(callback) = &self.bind_attr_callback {
            callback();
        }        
        // Link the program.
        ctx.link_program(pid);
        
        #[cfg(debug_assertions)]
        {
            // If debug build, print out any diagnostic info that just happened.
            if let Some(log) = ctx.get_program_info_log(pid) {
                if !log.is_empty() {
                    let msg = format!("[ {} ]\n", self.name);
                    console::log_2(&msg.into(), &log.into());
                }
            }
        }
        // Check the link status and exit with 'false' if there was a failure.
        let status = ctx.get_program_parameter(pid, Ctx::LINK_STATUS);
        if status.is_falsy() {
            self.delete_id();
            false
        } else {
            self.set_var_vectors();
            true
        }
    }
    fn set_var_vectors(&mut self) {
        use WebGlRenderingContext as Ctx;
        
        let ref pid = self.pid.as_ref().unwrap().clone();
        let     ctx = self.context.clone();

        
        // Get the number of attributes and add them to the attribute array.
        let nattr = ctx.get_program_parameter(pid, Ctx::ACTIVE_ATTRIBUTES);
        let nattr = nattr.as_f64().unwrap() as u32;
        
        for i in 0..nattr {
            let attrib = ctx.get_active_attrib(pid, i).unwrap();
            self.add_vertex_attr(attrib.name(), attrib.type_());
        }
        // Get the number of uniforms and add them to the uniform array.
        let nuni = ctx.get_program_parameter(pid, Ctx::ACTIVE_UNIFORMS);
        let nuni = nuni.as_f64().unwrap() as u32;
        
        for i in 0..nuni {
            let uni = ctx.get_active_uniform(pid, i).unwrap();
            self.add_uniform(uni.name(), uni.type_());
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.delete_id();
    }
}

