//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/decompress/generic.h
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


// SPDX-License-Identifier: GPL-2.0
// inbuf   - input buffer
// len     - len of pre-read data in inbuf
// fill    - function to fill inbuf when empty
// flush   - function to write out outbuf
// outbuf  - output buffer
// posp    - if non-null, input position (number of bytes read) will be
// returned here
//
// If len != 0, inbuf should contain all the necessary input data, and fill
// should be NULL
// If len = 0, inbuf can be NULL, in which case the decompressor will allocate
// the input buffer.  If inbuf != NULL it must be at least XXX_IOBUF_SIZE bytes.
// fill will be called (repeatedly...) to read data, at most XXX_IOBUF_SIZE
// bytes should be read per call.  Replace XXX with the appropriate decompressor
// name, i.e. LZMA_IOBUF_SIZE.
//
// If flush = NULL, outbuf must be large enough to buffer all the expected
// output.  If flush != NULL, the output buffer will be allocated by the
// decompressor (outbuf = NULL), and the flush function will be called to
// flush the output buffer at the appropriate time (decompressor and stream
// dependent).
//
// Utility routine to detect the decompression method
