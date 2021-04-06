
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use js_sys::Object;

use std::error::Error;
use std::fmt;

type OptInnerError = Option<Box<dyn Error + 'static>>;

#[derive(Debug)]
pub enum GfxError {
    HttpResponseNotOK ( String ),
    JSError {
        jsval : JsValue,
    },
    ResourceLoadError { 
        msg   : String, 
        inner : OptInnerError, 
    },
    MemoryError(crate::memory::MemoryError),
}

impl GfxError {
    pub fn new_http_rsp_not_ok(msg: String) -> Self {
        GfxError::HttpResponseNotOK(msg)
    }
    pub fn new_resource_load_error(msg    : String, 
                                   source : OptInnerError) -> Self
    {
        GfxError::ResourceLoadError {
            msg,
            inner: source,
        }
    }
    pub fn new_jserror(jsval: JsValue) -> Self 
    {
        GfxError::JSError {
            jsval
        }
    }
}

impl fmt::Display for GfxError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        use GfxError::*;
        
        match self {
            HttpResponseNotOK ( msg ) => {
                write!(f, "{}", msg)
            },
            JSError { jsval } => {
                let s: String = jsval.clone().dyn_into::<Object>()
                                             .unwrap()
                                             .to_string()
                                             .into();
                write!(f, "{}", s)
            },
            ResourceLoadError { msg, inner: _ } => { 
                write!(f, "{}", msg) 
            },
            MemoryError(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl Error for GfxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use GfxError::*;
        
        match self {
            ResourceLoadError { msg: _, inner: Some(src) } => {
                Some(src.as_ref())
            },
            MemoryError(e) => {
                Some(e)
            }
            _ => { None },
        }
    }
}

impl From<crate::memory::MemoryError> for GfxError {
    fn from(e: crate::memory::MemoryError) -> Self {
        GfxError::MemoryError(e)
    }
}


