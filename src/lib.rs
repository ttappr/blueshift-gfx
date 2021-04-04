#![allow(unused_must_use, unused_imports, dead_code, unused_variables)]

mod error;
mod gfx;
mod matrix;
mod memory;
mod program;
mod shader;
mod types;
mod utils;
mod vector;

use crate::gfx::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


