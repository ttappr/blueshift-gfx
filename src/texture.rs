
use std::any::Any;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use web_sys::WebglCompressedTexturePvrtc;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlTexture;

use crate::error::GfxError;
use crate::memory::Memory;
use crate::memory as memory;

const TEXTURE_CLAMP         : u32 = 1 << 0;
const TEXTURE_MIPMAP        : u32 = 1 << 1;
const TEXTURE_16_BITS       : u32 = 1 << 2;
const TEXTURE_16_BITS_5551  : u32 = 1 << 3;

const TEXTURE_FILTER_0X     : u32 = 0;
const TEXTURE_FILTER_1X     : u32 = 1;
const TEXTURE_FILTER_2X     : u32 = 2;
const TEXTURE_FILTER_3X     : u32 = 3;

const PVR_IDENTIFIER        : [u8;4] = *b"PVR!";

//#[repr(packed)] isn't needed since all fields are u32.
#[repr(C)]
struct PvrHeader {
    hdr_size        : u32,
    height          : u32,
    width           : u32,
    n_mipmap        : u32,
    flags           : u32,
    data_size       : u32,
    bpp             : u32,
    bit_red         : u32,
    bit_green       : u32,
    bit_blue        : u32,
    bit_alpha       : u32,
    tag             : u32,
    n_surface       : u32,
}

struct Texture {
    name            : String,
    tid             : Option<WebGlTexture>,
    width           : u16,
    height          : u16,
    bytes           : u8,
    size            : u32,
    target          : u32,
    internal_format : u32,
    format          : u32,
    texel_type      : u32,
    texel_array     : Vec<u8>,
    n_mipmap        : u32,
    compression     : u32,
    context         : Arc<WebGlRenderingContext>,
}

impl Texture {
    pub async fn new(name                 : &str, 
                     url                  : &str, 
                     flags                : u32,
                     filter               : u8,
                     anisotropic_filter   : f32,
                     context              : Arc<WebGlRenderingContext>
                    ) -> Result<Self, GfxError>
    {
        use WebGlRenderingContext as Ctx;
        let m = Memory::mopen(url).await?;
        Ok (
            Texture {
                name            : name.into(),
                tid             : None,
                width           : 0,
                height          : 0,
                bytes           : 0,
                size            : 0,
                target          : Ctx::TEXTURE_2D,
                internal_format : 0,
                format          : 0,
                texel_type      : 0,
                texel_array     : vec![],
                n_mipmap        : 0,
                compression     : 0,
                context
            })
    }
    fn load(&mut self, m: &Memory) {
    
    }
    fn load_png(&self, memory: &Memory) {
    
    }
    fn load_pvr(&mut self, memory: &Memory) -> Result<(), TextureError> {
        use WebGlRenderingContext       as CTX;
        use WebglCompressedTexturePvrtc as CTP;
        use TextureError::*;
        
        const PVRTC2: u8 = 24;
        const PVRTC4: u8 = 25;
        
        let header = memory.bytes_copy_into_new::<PvrHeader>()?;

        for i in 0..4 {
            if ((header.tag >> (i * 8)) & 0xFF) as u8 != PVR_IDENTIFIER[i] {
                let msg = "PVR texture file has bad identifier field.";
                Err( HeaderFormatError(msg.into()) )?
            }
        }
        
        let ver = (header.flags & 0xFF) as u8;
        
        if ver == PVRTC2 || ver == PVRTC4 {
            self.width    = header.width  as u16;
            self.height   = header.height as u16;
            self.bytes    = header.bpp    as u8;
            self.n_mipmap = header.n_mipmap + 1;
            
            self.compression = {
                if header.bit_alpha != 0 {
                    if header.bpp == 4 {
                        CTP::COMPRESSED_RGBA_PVRTC_4BPPV1_IMG
                    } else {
                        CTP::COMPRESSED_RGBA_PVRTC_2BPPV1_IMG
                    }
                } else {
                    if header.bpp == 4 {
                        CTP::COMPRESSED_RGB_PVRTC_4BPPV1_IMG
                    } else {
                        CTP::COMPRESSED_RGB_PVRTC_2BPPV1_IMG
                    }
                }
            };
            let hdr_size  = std::mem::size_of::<PvrHeader>();
            let data_size = header.data_size as usize;
            let bytes     = memory.bytes();
            
            if bytes.len() != data_size {
                let msg = format!("PVR texture file has bad data_size field \
                                   value ({}). Computed size is ({}).",
                                   data_size, bytes.len() - hdr_size);                
                Err( HeaderFormatError(msg) )?
            } else {
                for i in hdr_size..data_size {
                    self.texel_array.push(bytes[i]);
                }
            }
        }
        Ok(())
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.context.delete_texture(self.tid.take().as_ref());
    }
}

#[derive(Debug)]
enum TextureError {
    HeaderFormatError(String),
    MemoryError(memory::MemoryError)
}

impl Error for TextureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use TextureError::*;
        match self {
            MemoryError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for TextureError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        use TextureError::*;
        match self {
            HeaderFormatError ( msg ) => {
                write!(f, "{}", msg)
            },
            MemoryError ( err ) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl From<memory::MemoryError> for TextureError {
    fn from(err: memory::MemoryError) -> Self {
        TextureError::MemoryError(err)
    }
}

