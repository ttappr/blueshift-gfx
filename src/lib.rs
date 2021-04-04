#![allow(unused_must_use, unused_imports, dead_code, unused_variables)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlShader;

mod error;
mod gfx;
mod matrix;
mod memory;
mod program;
mod shader;
mod types;
mod utils;
mod vector;

use crate::gfx::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, ch-1!");
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {

}

impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        Universe {}
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let doc     = web_sys::window().unwrap().document().unwrap();
    let canvas  = doc.get_element_by_id("canvas")
                     .unwrap()
                     .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas.get_context("webgl")?
                        .unwrap()
                        .dyn_into::<WebGlRenderingContext>()?;
    Ok(())
}
