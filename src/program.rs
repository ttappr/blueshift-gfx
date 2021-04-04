
use std::sync::Arc;

//use web_sys::console;
use web_sys::WebGlRenderingContext;
use wasm_bindgen_futures::JsFuture;

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

pub type ProgramDrawCallback        = dyn Fn();
pub type ProgramBindAttribCallback  = dyn Fn();

pub struct Program {
    name                : String,
    vertex_shader       : Shader,
    fragment_shader     : Shader,
    pid                 : u32,
    uniform_count       : u8,
    uniform_array       : Vec<Uniform>,
    vertex_attrib_count : u8,
    vertex_attrib_array : Vec<VertexAttrib>,
    draw_callback       : Box<ProgramDrawCallback>,
    bind_attr_callback  : Box<ProgramBindAttribCallback>,
    context             : Arc<WebGlRenderingContext>,
}

impl Program {
    
    pub async fn new(name                 : String,
                     vertex_shader_url    : String,
                     fragment_shader_url  : String,
                     relative_path        : bool,
                     debug_shader         : bool,
                     bind_attr_callback   : Box<ProgramBindAttribCallback>,
                     draw_callback        : Box<ProgramDrawCallback>,
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
                name                : name,
                vertex_shader       : vert_shader,
                fragment_shader     : frag_shader,
                pid                 : 0,
                uniform_count       : 0,
                uniform_array       : vec![],
                vertex_attrib_count : 0,
                vertex_attrib_array : vec![],
                draw_callback,
                bind_attr_callback,
                context             : context,
            } )
    }
}
