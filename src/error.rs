
use wasm_bindgen::prelude::*;

use std::error::Error;
use std::fmt;

type OptInnerError = Option<Box<dyn Error + 'static>>;

#[derive(Debug)]
pub enum GfxError {
    ResourceLoadError { 
        msg   : String, 
        inner : OptInnerError, 
    },
}

impl GfxError {
    pub fn new_resource_load_error(msg   : &str, 
                                   inner : OptInnerError
                                  ) -> Self 
    {
        use GfxError::ResourceLoadError;
        ResourceLoadError { msg: msg.to_string(), inner }
    }
}

impl fmt::Display for GfxError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        use GfxError::*;
        
        match self {
            ResourceLoadError { msg, inner: _ } => { write!(f, "blah!") },
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
            _ => { None },
        }
    }
}

impl From<JsValue> for GfxError {
    fn from(err: JsValue) -> Self {
        GfxError::ResourceLoadError { 
            msg   : err.as_string().unwrap(), 
            inner : None,
        }
    }
}

