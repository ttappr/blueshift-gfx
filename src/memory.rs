

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
    pub async fn mopen(url: &str) -> Result<Self, GfxError> 
    {
        let window = web_sys::window().unwrap();
        let url    = url.to_string();
        
        let rsp = JsFuture::from(window.fetch_with_str(&url)).await?;
        let rsp = rsp.dyn_into::<Response>().unwrap();
        
        if !rsp.ok() {
            let msg = format!("Fetch response for {} reported status {}.", 
                              url, rsp.status());
            return Err( GfxError::new_http_rsp_not_ok(msg) );
        }
        
        let buf = JsFuture::from(rsp.array_buffer()?).await?;
        
        //let buf = buf.dyn_into::<ArrayBuffer>().unwrap();        
        
        // rsp.blob()? could be used instead of .array_buffer(), then
        // maybe use a ReadableStream::from(blob) to get the data.
        
        let u8vec: Vec<u8> = js_sys::Uint8Array::new(&buf).to_vec();
        
        Ok( Memory { url, size: u8vec.len(), position: 0, buffer: u8vec } )
    }
    pub fn as_str(&self) -> &str 
    {
        std::str::from_utf8(&self.buffer)
                  .expect("Buffer doesn't hold a valid utf-8 string.")
    }
}

