
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

#[derive(Debug)]
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
            Err( FetchStatusError(url.clone(), 
                                  rsp.status(), 
                                  rsp.status_text()) )?;
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
    pub fn bytes(&self) -> &[u8] {
        &self.buffer
    }
    pub unsafe fn bytes_as_ref<T>(&self) -> Result<&T, MemoryError> {
        use MemoryError::RefTransmuteError;
        if std::mem::size_of::<T>() > self.buffer.len() {
            Err( RefTransmuteError(std::any::type_name::<T>().into(),
                                   "Target type size is larger than the number \
                                   of bytes in the memory buffer.".into()) )
        } else {
            Ok( std::mem::transmute::<*const u8, &T>(self.buffer.as_ptr()) )
        }
    }
    pub fn bytes_copy_into_new<T>(&self) -> Result<T, MemoryError> {
        use MemoryError::RefTransmuteError;
        if std::mem::size_of::<T>() > self.buffer.len() {
            Err( RefTransmuteError(std::any::type_name::<T>().into(),
                                   "Target type size is larger than the number \
                                   of bytes in the memory buffer.".into()) )
        } else {
            Ok( unsafe { std::mem::transmute_copy(&self.buffer) } )
        }
    }
}

#[derive(Debug)]
pub enum MemoryError {
    FetchError(String, JsValue),
    FetchStatusError(String, u16, String),
    DataError(JsValue),
    RefTransmuteError(String, String),
}

impl Error for MemoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl fmt::Display for MemoryError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        use MemoryError::*;
        match self {
            FetchError(url, jsval) => {
                write!(f, "Fetch from ({}) failed with error ({}).", 
                       url, jsval_to_string(&jsval))
            },
            FetchStatusError(url, status, status_text) => {
                write!(f, "Fetch response for ({}) reported status {} ({})", 
                       url, status, status_text)
            },
            DataError(jsval) => {
                write!(f, "Extracting ArrayBuffer from fetched data \
                       failed with error ({}).", 
                       jsval_to_string(&jsval))
            },
            RefTransmuteError(type_name, msg) => {
                write!(f, "Error converting buffer from &[u8] to {}; {}", 
                       type_name, msg)
            },
        }
    }
}

#[cfg(feature = "test_accessors")]
impl MemoryError {
    pub fn status(&self) -> u16 {
        use MemoryError::*;
        match self {
            FetchStatusError(_, stat, _) => *stat,
            _ => panic!("{:?} doesn't have status property.", self),
        }
    }
    pub fn jsvalue(&self) -> &JsValue {
        use MemoryError::*;
        match self {
            FetchError(_, v) => &v,
            DataError(v) => &v,
            _ => panic!("{:?} doesn't have an associated JsValue.", self),
        }
    }
    pub fn jsvalue_as_string(&self) -> String {
        jsval_to_string(self.jsvalue())
    }
    pub fn url(&self) -> &str {
        use MemoryError::*;
        match self {
            FetchError(url, _) => &url,
            FetchStatusError(url, _, _) => &url,
            _ => panic!("{:?} doesn't have an associated URL.", self),
        }
    }
}

