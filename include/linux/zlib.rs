//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/zlib.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// zlib.h -- interface of the 'zlib' general purpose compression library
//

// zlib deflate based on ZLIB_VERSION "1.1.3"
// zlib inflate based on ZLIB_VERSION "1.2.3"
//
// Z_PACKET_FLUSH is added and used by ppp_deflate. Before returning
// Addition of zlib_inflateIncomp which copies incompressible data into
//
// constants
pub const Z_NO_FLUSH: c_int = 0;

pub const Z_PACKET_FLUSH: c_int = 2;
pub const Z_SYNC_FLUSH: c_int = 3;
pub const Z_FULL_FLUSH: c_int = 4;
pub const Z_FINISH: c_int = 5;

// Allowed flush values; see deflate() and inflate() below for details
pub const Z_OK: c_int = 0;
pub const Z_STREAM_END: c_int = 1;
pub const Z_NEED_DICT: c_int = 2;

// Return codes for the compression/decompression functions. Negative
// values are errors, positive values are used for special but normal events.
//
pub const Z_NO_COMPRESSION: c_int = 0;
pub const Z_BEST_SPEED: c_int = 1;
pub const Z_BEST_COMPRESSION: c_int = 9;

// compression levels
pub const Z_FILTERED: c_int = 1;
pub const Z_HUFFMAN_ONLY: c_int = 2;
pub const Z_DEFAULT_STRATEGY: c_int = 0;
// compression strategy; see deflateInit2() below for details
pub const Z_BINARY: c_int = 0;
pub const Z_ASCII: c_int = 1;
pub const Z_UNKNOWN: c_int = 2;
// Possible values of the data_type field
pub const Z_DEFLATED: c_int = 8;
// The deflate compression method (the only one supported in this version)
// basic functions
extern "C" {
    pub fn zlib_deflate_workspacesize(windowBits: c_int, memLevel: c_int) -> c_int;
}
//
extern "C" {
    pub fn zlib_deflate_dfltcc_enabled() -> c_int;
}
//
extern "C" {
    pub fn deflateInit(strm: z_streamp, level: c_int) -> c_int;
}
//
extern "C" {
    pub fn zlib_deflate(strm: z_streamp, flush: c_int) -> c_int;
}
//
extern "C" {
    pub fn zlib_deflateEnd(strm: z_streamp) -> c_int;
}
//
extern "C" {
    pub fn zlib_inflate_workspacesize() -> c_int;
}
//
extern "C" {
    pub fn zlib_inflateInit(strm: z_streamp) -> c_int;
}
//
extern "C" {
    pub fn zlib_inflate(strm: z_streamp, flush: c_int) -> c_int;
}
//
extern "C" {
    pub fn zlib_inflateEnd(strm: z_streamp) -> c_int;
}
//
// Advanced functions
//
extern "C" {
    pub fn zlib_deflateReset(strm: z_streamp) -> c_int;
}
//
extern "C" {
    pub fn inflateInit2(strm: z_streamp, windowBits: c_int) -> c_int;
}
//
extern "C" {
    pub fn zlib_inflateReset(strm: z_streamp) -> c_int;
}
//
extern "C" {
    pub fn zlib_inflateIncomp(strm: *mut z_stream) -> c_int;
}
//

extern "C" {
    pub fn zlib_inflateInit2(strm: z_streamp, windowBits: c_int) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct internal_state {

// Utility function: initialize zlib, unpack binary blob, clean up zlib,
// return len or negative error code.
    pub src_sz): *const *const *const extern int zlib_inflate_blob(void dst, unsigned dst_sz, void src, unsigned,
