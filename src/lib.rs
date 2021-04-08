#![allow(unused_must_use, unused_imports, dead_code, unused_variables)]

mod error;
mod gfx;
mod matrix;
mod memory;
mod program;
mod shader;
mod texture;
mod types;
mod utils;
mod vector;

pub use crate::error::*;
pub use crate::gfx::*;
pub use crate::matrix::*;
pub use crate::memory::*;
pub use crate::program::*;
pub use crate::shader::*;
pub use crate::texture::*;
pub use crate::types::*;
pub use crate::utils::*;
pub use crate::vector::*;



#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


