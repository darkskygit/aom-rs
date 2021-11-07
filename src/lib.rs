#![allow(non_upper_case_globals)]

extern crate av_data as data;
extern crate libaom_sys as ffi;

#[cfg(feature = "codec-trait")]
extern crate av_codec as codec;

pub mod common;
pub mod decoder;
pub mod encoder;
