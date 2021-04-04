#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::console;
use web_sys::WebGlRenderingContext;

use js_sys::Math::{sin, cos};

use crate::console_log;
use crate::matrix::*;
use crate::types::DEG_TO_RAD;
use crate::vector::*;

const MAX_MODELVIEW_MATRIX  : usize = 8;
const MAX_PROJECTION_MATRIX : usize = 2;
const MAX_TEXTURE_MATRIX    : usize = 2;

enum MatrixMode {
    ModelView  = 0,
    Projection = 1,
    Texture    = 2,
}

struct Gfx {
    matrix_mode                 : MatrixMode,
    modelview_matrix_index      : usize,
    projection_matrix_index     : usize,
    texture_matrix_index        : usize,
    modelview_matrix            : [Mat4; MAX_MODELVIEW_MATRIX  ],
    projection_matrix           : [Mat4; MAX_PROJECTION_MATRIX ],
    texture_matrix              : [Mat4; MAX_TEXTURE_MATRIX    ],
    modelview_projection_matrix : Mat4,
    normal_matrix               : Mat3,
    context                     : WebGlRenderingContext,
}

impl Gfx {
    pub fn new(canvas_selector: &str) -> Result<Self, JsValue> 
    {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas   = document.query_selector(canvas_selector)?
                               .unwrap()
                               .dyn_into::<web_sys::HtmlCanvasElement>()?;
        let context  = canvas.get_context("webgl")?
                             .unwrap()
                             .dyn_into::<WebGlRenderingContext>()?;
        let mut gfx = Gfx {
                matrix_mode                 : MatrixMode::ModelView,
                modelview_matrix_index      : 0,
                projection_matrix_index     : 0,
                texture_matrix_index        : 0,
                modelview_matrix            : Default::default(),
                projection_matrix           : Default::default(),
                texture_matrix              : Default::default(),
                modelview_projection_matrix : Mat4::new(),
                normal_matrix               : Mat3::new(),
                context
        };
        gfx.start();
        Ok(gfx)
    }
    fn start(&mut self) -> Result<(), JsValue> 
    {
        use web_sys::WebGlRenderingContext as GLRendCtx;
        
        let context = &self.context;
        
        console::log_2(&"GL_VENDOR     : ".into(), 
                       &context.get_parameter(GLRendCtx::VENDOR)
                       .unwrap());
        console::log_2(&"GL_RENDERER   : ".into(), 
                       &context.get_parameter(GLRendCtx::RENDERER)
                       .unwrap());
        console::log_2(&"GL_VERSION    : ".into(), 
                       &context.get_parameter(GLRendCtx::VERSION)
                       .unwrap());
        console::log_2(&"GL_EXTENSIONS : ".into(), 
                       &context.get_supported_extensions()
                       .unwrap());

        context.hint(GLRendCtx::GENERATE_MIPMAP_HINT, 
                     GLRendCtx::NICEST);
        //context.hint(GLRendCtx::FRAGMENT_SHADER_DERIVATIVE_HINT_OES, 
        //             GLRendCtx::NICEST);
        
        context.enable(GLRendCtx::DEPTH_TEST);
        context.enable(GLRendCtx::CULL_FACE);
        context.disable(GLRendCtx::DITHER);
        
        context.depth_mask(true);
        context.depth_func(GLRendCtx::LESS);
        context.depth_range(0.0, 1.0);
        context.clear_depth(1.0);
        context.cull_face(GLRendCtx::BACK);
        context.front_face(GLRendCtx::CCW);
        context.clear_stencil(0);
        context.stencil_mask(0xFFFFFFFF);
        
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(GLRendCtx::DEPTH_BUFFER_BIT   | 
                      GLRendCtx::STENCIL_BUFFER_BIT |
                      GLRendCtx::COLOR_BUFFER_BIT   );
                      
        self.set_matrix_mode(MatrixMode::Texture);
        self.load_identity();
        
        self.set_matrix_mode(MatrixMode::Projection);
        self.load_identity();
        
        self.set_matrix_mode(MatrixMode::ModelView);
        self.load_identity();
        
        self.log_errors();
        
        Ok(())
    }
    fn set_matrix_mode(&mut self, mode: MatrixMode) {
        self.matrix_mode = mode;
    }
    fn load_identity(&mut self) {
        match self.matrix_mode {
            MatrixMode::ModelView => { 
                self.get_modelview_matrix_mut().identity(); 
            },
            MatrixMode::Projection => { 
                self.get_projection_matrix_mut().identity(); 
            },
            MatrixMode::Texture => { 
                self.get_texture_matrix_mut().identity();
            },
        }
    }
    fn get_modelview_matrix_mut(&mut self) -> &mut Mat4 {
        &mut self.modelview_matrix[self.modelview_matrix_index]
    }
    fn get_modelview_matrix(&self) -> &Mat4 {
        &self.modelview_matrix[self.modelview_matrix_index]
    }
    fn get_projection_matrix_mut(&mut self) -> &mut Mat4 {
        &mut self.projection_matrix[self.projection_matrix_index]
    }
    fn get_projection_matrix(&self) -> &Mat4 {
        &self.projection_matrix[self.projection_matrix_index]
    }
    fn get_texture_matrix_mut(&mut self) -> &mut Mat4 {
        &mut self.texture_matrix[self.texture_matrix_index]
    }
    fn get_texture_matrix(&self) -> &Mat4 {
        &self.texture_matrix[self.texture_matrix_index]
    }
    fn log_errors(&self) {
        use web_sys::WebGlRenderingContext as GLRendCtx;
        let mut errors = vec![];
        
        loop {
            let error = self.context.get_error();
            match error {
                GLRendCtx::INVALID_ENUM => {
                    errors.push("GL_INVALID_ENUM");
                },
                GLRendCtx::INVALID_VALUE => {
                    errors.push("GL_INVALID_VALUE");
                },
                GLRendCtx::INVALID_OPERATION => {
                    errors.push("GL_INVALID_OPERATION");
                },
                GLRendCtx::OUT_OF_MEMORY => {
                    errors.push("GL_OUT_OF_MEMORY");
                },
                GLRendCtx::NO_ERROR => {
                    break;
                },
                _ => { 
                    errors.push("UNSPECIFIED");
                },
            }
        }
        if !errors.is_empty() {
            let mut estring = errors.join(", ");
            estring.insert_str(0, "GL_ERRORS: ");
            console::log_1(&estring.into());
        }
    }
    fn push_matrix(&mut self) {
        match self.matrix_mode {
            MatrixMode::ModelView => {
                let i = self.modelview_matrix_index;
                let a = self.modelview_matrix[i];
                let b = &mut self.modelview_matrix[i + 1];
                a.copy_to(b);
                self.modelview_matrix_index += 1;
            },
            MatrixMode::Projection => {
                let i = self.projection_matrix_index;
                let a = self.projection_matrix[i];
                let b = &mut self.projection_matrix[i + 1];
                a.copy_to(b);
                self.projection_matrix_index += 1;
            },
            MatrixMode::Texture => {
                let i = self.texture_matrix_index;
                let a = self.texture_matrix[i];
                let b = &mut self.texture_matrix[i + 1];
                a.copy_to(b);
                self.texture_matrix_index += 1;
            }
        }
    }
    fn pop_matrix(&mut self) {
        match self.matrix_mode {
            MatrixMode::ModelView => {
                self.modelview_matrix_index -= 1;
            },
            MatrixMode::Projection => {
                self.projection_matrix_index -= 1;
            },
            MatrixMode::Texture => {
                self.texture_matrix_index -= 1;
            }
        }
    }
    fn load_matrix(&mut self, m: &Mat4) {
        match self.matrix_mode {
            MatrixMode::ModelView => {
                m.copy_to(self.get_modelview_matrix_mut());
            },
            MatrixMode::Projection => {
                m.copy_to(self.get_projection_matrix_mut());
            },
            MatrixMode::Texture => {
                m.copy_to(self.get_texture_matrix_mut());
            }
        }
    }
    fn multiply_matrix(&mut self, m: &Mat4) {
        match self.matrix_mode {
            MatrixMode::ModelView => {
                let result = self.get_modelview_matrix().multiply(m);
                *self.get_modelview_matrix_mut() = result;
            },
            MatrixMode::Projection => {
                let result = self.get_projection_matrix().multiply(m);
                *self.get_projection_matrix_mut() = result;
            },
            MatrixMode::Texture => {
                let result = self.get_texture_matrix().multiply(m);
                *self.get_texture_matrix_mut() = result;
            }
        }
    }
    fn translate(&mut self, x: f32, y: f32, z: f32) {
        let v = Vec3::new(x, y, z);
        
        match self.matrix_mode {
            MatrixMode::ModelView => {
                self.get_modelview_matrix_mut().translate(&v);
            },
            MatrixMode::Projection => {
                self.get_projection_matrix_mut().translate(&v);
            },
            MatrixMode::Texture => {
                self.get_texture_matrix_mut().translate(&v);
            }
        }
    }
    fn rotate(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        if angle == 0.0 { return; }
        let v = Vec4::new(x, y, z, angle);
        match self.matrix_mode {
            MatrixMode::ModelView => {
                self.get_modelview_matrix_mut().rotate(&v);
            },
            MatrixMode::Projection => {
                self.get_projection_matrix_mut().rotate(&v);
            },
            MatrixMode::Texture => {
                self.get_texture_matrix_mut().rotate(&v);
            }
        }
    }
    fn scale(&mut self, x: f32, y: f32, z: f32) {
        let scale = Vec3::new(1.0, 1.0, 1.0);
        let v     = Vec3::new(x, y, z);
        
        match self.matrix_mode {
            MatrixMode::ModelView => {
                self.get_modelview_matrix_mut().scale(&v);
            },
            MatrixMode::Projection => {
                self.get_projection_matrix_mut().scale(&v);
            },
            MatrixMode::Texture => {
                self.get_texture_matrix_mut().scale(&v);
            }
        }
    }
    fn get_modelview_projection_matrix(&mut self) -> &Mat4 {
        // TODO - Make sure this is performant. It should be since the structs
        //        being copied are small.
        let proj_mtx = self.get_projection_matrix();
        let modv_mtx = self.get_modelview_matrix();
        self.modelview_projection_matrix = proj_mtx.multiply(&modv_mtx);
        &self.modelview_projection_matrix
    }
    fn get_normal_matrix(&mut self) -> &Mat3 {
        let mut mat = Mat4::new();
        self.get_modelview_matrix().copy_to(&mut mat);
        mat.invert_full();
        mat.transpose();
        mat.copy_to_mat3(&mut self.normal_matrix);
        &self.normal_matrix
    }
    fn ortho(&mut self, 
             left       : f32,
             right      : f32,
             bottom     : f32,
             top        : f32,
             clip_start : f32,
             clip_end   : f32) 
    {
        match self.matrix_mode {
            MatrixMode::ModelView => {
                self.get_modelview_matrix_mut().ortho(left,       right, 
                                                      bottom,     top, 
                                                      clip_start, clip_end);
            },
            MatrixMode::Projection => {
                self.get_projection_matrix_mut().ortho(left,       right, 
                                                       bottom,     top, 
                                                       clip_start, clip_end);
            },
            MatrixMode::Texture => {
                self.get_texture_matrix_mut().ortho(left,       right, 
                                                    bottom,     top, 
                                                    clip_start, clip_end);
            }
        }
    }
    fn set_orthographic_2d(&mut self,
                           left     : f32,
                           right    : f32,
                           bottom   : f32,
                           top      : f32)
    {
        self.ortho(left, right, bottom, top, -1.0, 1.0);
    }
    fn set_orthographic(&mut self,
                        screen_ratio        : f32,
                        scale               : f32,
                        aspect_ratio        : f32,
                        clip_start          : f32,
                        clip_end            : f32,
                        screen_orientation  : f32)
    {
        let scale = (scale * 0.5) * aspect_ratio;
        self.ortho(-1.0,          1.0, 
                   -screen_ratio, screen_ratio, 
                   clip_start,    clip_end);
        self.scale(1.0 / scale, 1.0 / scale, 1.0);
        if screen_orientation != 0.0 {
            self.rotate(screen_orientation, 0.0, 0.0, 1.0);
        }
    }
    fn set_perspective(&mut self,
                       fovy                 : f32,
                       aspect_ratio         : f32,
                       clip_start           : f32,
                       clip_end             : f32,
                       screen_orientation   : f32)
    {
        let d = clip_end - clip_start;
        let r = (fovy * 0.5) * DEG_TO_RAD;
        let s = sin(r as f64) as f32;
        let c = cos(r as f64) as f32 / s;
        
        let mut mat = Mat4::new_identity();
        
        mat.m[0].x = c / aspect_ratio;
        mat.m[1].y = c;
        mat.m[2].z = -(clip_end - clip_start) / d;
        mat.m[2].w = -1.0;
        mat.m[3].z = -2.0 * (clip_start * clip_end) / d;
        mat.m[3].w = 0.0;
        
        self.multiply_matrix(&mat);
        
        if screen_orientation != 0.0 {
            self.rotate(screen_orientation, 0.0, 0.0, 1.0);
        }
    }
    fn look_at(&mut self,
               eye      : &Vec3,
               center   : &Vec3,
               up       : &Vec3)
    {
        let (mut f, mut s, u);
        
        f = center.diff(eye);
        f.normalize();

        s = f.cross(up);
        s.normalize();

        u = s.cross(&f);
        
        let mut mat = Mat4::new_identity();
        
        mat.m[0].x = s.x;
        mat.m[1].x = s.y;
        mat.m[2].x = s.z;
        
        mat.m[0].y = u.x;
        mat.m[1].y = u.y;
        mat.m[2].y = u.z;
        
        mat.m[0].z = -f.x;
        mat.m[1].z = -f.y;
        mat.m[2].z = -f.z;
        
        self.multiply_matrix(&mat);
        
        self.translate(-eye.x, -eye.y, -eye.z);
    }
    fn project(objx              : f32,
               objy              : f32,
               objz              : f32,
               modelview_matrix  : &Mat4,
               projection_matrix : &Mat4,
               viewport_matrix   : &[i32]
              ) -> Option<Vec3>
    {
        let mut vin  = Vec4::new(objx, objy, objz, 1.0);
        let     vout = vin.multiply_mat4(modelview_matrix);
        
        vin = vout.multiply_mat4(projection_matrix);
        
        if vin.w == 0.0 {
            None
        } else {
            vin.x /= vin.w;
            vin.y /= vin.w;
            vin.z /= vin.w;
            
            vin.x = vin.x * 0.5 + 0.5;
            vin.y = vin.y * 0.5 + 0.5;
            vin.z = vin.z * 0.5 + 0.5;
            
            vin.x = vin.x * viewport_matrix[2] as f32 + 
                            viewport_matrix[0] as f32;
            vin.y = vin.y * viewport_matrix[3] as f32 + 
                            viewport_matrix[1] as f32;
            
            Some(vin.into())
        }
    }
    fn unproject(winx              : f32,
                 winy              : f32,
                 winz              : f32,
                 modelview_matrix  : &Mat4,
                 projection_matrix : &Mat4,
                 viewport_matrix   : &[i32]
                ) -> Option<Vec3>
    {
        let mut fin = projection_matrix.multiply(modelview_matrix);
        fin.invert_full();
        
        let mut vin = Vec4::new(winx, winy, winz, 1.0);
        
        vin.x = (vin.x - viewport_matrix[0] as f32) / viewport_matrix[2] as f32;
        vin.y = (vin.y - viewport_matrix[1] as f32) / viewport_matrix[3] as f32;
        
        vin.x = vin.x * 2.0 - 1.0;
        vin.y = vin.y * 2.0 - 1.0;
        vin.z = vin.z * 2.0 - 1.0;
        
        let mut vout = vin.multiply_mat4(&fin);
        
        if vout.w == 0.0 {
            None
        } else {
            vout.x /= vout.w;
            vout.y /= vout.w;
            vout.z /= vout.w;
            
            Some(vout.into())
        }
    }
}










