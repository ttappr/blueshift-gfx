//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
#![cfg(feature = "test_accessors")]

use std::sync::Arc;

use wasm_bindgen::JsCast;

use web_sys::console;
use web_sys::WebGlRenderingContext;

use blueshift_gfx::Program;

use wasm_bindgen_test::*;

//mod web;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub async fn program_new() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let canvas =  document.create_element("canvas")
                          .unwrap()
                          .dyn_into::<web_sys::HtmlCanvasElement>()
                          .expect("Failed to get canvas.");

    let context =   canvas.get_context("webgl")
                          .unwrap()
                          .unwrap()
                          .dyn_into::<WebGlRenderingContext>()
                          .expect("Failed to get context.");
                             
    match Program::new("foo-program".into(),
                       "http://localhost:8000/tests/vertex.glsl".into(),
                       "http://localhost:8000/tests/fragment.glsl".into(),
                        None, None,
                        Arc::new(context)).await
    {
        Ok(mut p) => {
            console::log_1(&"A Program object was created!".into());
            assert!(p.link());
            console::log_1(&"The Program was successfully linked!".into());
        },
        Err(e) => {
            panic!("Program creation error: {}", e.to_string());
        }
    }
}













