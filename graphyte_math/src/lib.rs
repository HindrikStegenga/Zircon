#![no_std]
extern crate alloc;

pub mod vector_generic;

mod matrix;
mod vector;

pub use matrix::*;
pub use vector::*;
