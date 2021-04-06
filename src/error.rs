
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
}

impl GfxError {
    pub fn new_http_rsp_not_ok(msg: String) -> Self {
        GfxError::HttpResponseNotOK(msg)
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
    fn from(jsval: JsValue) -> Self {
        // TODO - Add match to branch on the type of the JsValue error to 
        //        produce the corresponding GfxError.
        
        //if err.is_instance_of::<>()
        
        GfxError::JSError { jsval }
    }
}


