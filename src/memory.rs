
use std::any::Any;
use std::error::Error;
use std::fmt;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
//use web_sys::{Request, RequestInit, RequestMode, Response};

use js_sys::ArrayBuffer;
use js_sys::Object;
use js_sys::TypeError;
use web_sys::Blob;
use web_sys::console;
use web_sys::Response;

use crate::error::*;
use crate::utils::jsval_to_string;

use MemoryError::*;

pub struct Memory {
    url         : String,
    size        : usize,
    position    : usize,
    buffer      : Vec<u8>,
}

impl Memory {

    pub async fn mopen(url: &str) -> Result<Self, MemoryError>
    {
        use MemoryError::*;
        
        let window = web_sys::window().unwrap();
        let url    = url.to_string();
        
        let rsp = JsFuture::from(window.fetch_with_str(&url)).await?;
        let rsp = rsp.dyn_into::<Response>().unwrap();
        
        if !rsp.ok() {
            let emsg = format!("Fetch response for {} reported status {}.", 
                               url, rsp.status());
            MemoryError::raise_error(&emsg)?;
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

#[derive(Debug)]
pub enum MemoryError {
    JSError(JsValue),
    NativeError(String),
}

impl Error for MemoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl fmt::Display for MemoryError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSError(jsval) => {
                write!(f, "{}", jsval_to_string(&jsval))
            },
            NativeError(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl From<JsValue> for MemoryError {
    fn from(jsval: JsValue) -> Self {
        if jsval.is_instance_of::<TypeError>() {
            console::log_1(&"FOUND TYPE ERROR".into());
        }
        JSError(jsval)
    }
}

impl MemoryError {
    fn raise_error(msg: &str) -> Result<(), MemoryError> {
        Err(NativeError(msg.to_string()))
    }
}


