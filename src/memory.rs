
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
        
        let rsp = JsFuture::from(window.fetch_with_str(&url))
                  .await
                  .map_err(|e| FetchError(url.clone(), e))?;
                  
        let rsp = rsp.dyn_into::<Response>().unwrap();
        
        if !rsp.ok() {
            MemoryError::raise_status_error(&url, 
                                            rsp.status(), 
                                            rsp.status_text())?;
        }
        let buf = rsp.array_buffer()        .map_err(|e| DataError(e))?;
        let buf = JsFuture::from(buf).await .map_err(|e| DataError(e))?;
        
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
    FetchError(String, JsValue),
    FetchStatusError(String, u16, String),
    DataError(JsValue),
}

impl Error for MemoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl fmt::Display for MemoryError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError(url, jsval) => {
                write!(f, "Fetch from ({}) failed with error ({}).", 
                       url, jsval_to_string(&jsval))
            }
            FetchStatusError(url, status, status_text) => {
                write!(f, "Fetch response for ({}) reported status {} ({})", 
                       url, status, status_text)
            }
            DataError(jsval) => {
                write!(f, "Extracting ArrayBuffer from fetched data \
                       failed with error ({}).", 
                       jsval_to_string(&jsval))
            }
        }
    }
}

impl MemoryError {
    fn raise_status_error(url         : &str, 
                          status      : u16, 
                          status_text : String
                         ) -> Result<(), MemoryError> 
    {
        Err(FetchStatusError(url.to_string(), status, status_text))
    }
}


