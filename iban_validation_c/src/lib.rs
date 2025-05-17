//! C bindings for the iban_validation_rs library
//!
//! This crate provides FFI functions that can be called from C code
//! to validate IBANs and extract bank and branch IDs.

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
extern crate alloc;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
mod ffi;

// Re-export the FFI functions at the crate root
pub use ffi::*;
