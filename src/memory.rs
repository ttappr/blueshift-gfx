

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
//use web_sys::{Request, RequestInit, RequestMode, Response};

use js_sys::ArrayBuffer;
use web_sys::Blob;
use web_sys::Response;

use crate::error::*;

pub struct Memory {
    url         : String,
    size        : usize,
    position    : usize,
    buffer      : Vec<u8>,
}

impl Memory {
    pub async fn mopen(url           : &str, 
                       relative_path : bool
                      ) -> Result<Self, GfxError> 
    {
        let window = web_sys::window().unwrap();
        let url    = url.to_string();
        
        let rsp = JsFuture::from(window.fetch_with_str(&url)).await?;
        let rsp = rsp.dyn_into::<Response>().unwrap();
        
        assert!(rsp.is_instance_of::<Response>());
        
        let buf = JsFuture::from(rsp.array_buffer()?).await?;
        
        //let buf = buf.dyn_into::<ArrayBuffer>().unwrap();        
        //assert!(buf.is_instance_of::<ArrayBuffer>());
        
        // rsp.blob()? could be used instead of .array_buffer(), then
        // maybe use a ReadableStream::from(blob) to get the data.
        
        let u8vec: Vec<u8> = js_sys::Uint8Array::new(&buf).to_vec();
        
        Ok( Memory { url, size: u8vec.len(), position: 0, buffer: u8vec } )
    }
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buffer)
                  .expect("Buffer doesn't hold a valid utf-8 string.")
    }
}

