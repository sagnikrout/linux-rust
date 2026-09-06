//! Automatically rewritten from C Header to Rust Module
//! Source: lib/xz/xz_private.h
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


// SPDX-License-Identifier: 0BSD
//
// Private includes and definitions
//
// Author: Lasse Collin <lasse.collin@tukaani.org>
//

// XZ_PREBOOT may be defined only via decompress_unxz.c.

//
// For userspace builds, use a separate header to define the required
// macros and functions. This makes it easier to adapt the code into
// different environments and avoids clutter in the Linux kernel tree.
//

// If no specific decoding mode is requested, enable support for all modes.

//
// The DEC_IS_foo(mode) macros are used in "if" statements. If only some
// of the supported modes are enabled, these macros will evaluate to true or
// false at compile time and thus allow the compiler to omit unneeded code.
//

//
// If any of the BCJ filter decoders are wanted, define XZ_DEC_BCJ.
// XZ_DEC_BCJ is used to enable generic support for BCJ decoders.
//

//
// Allocate memory for LZMA2 decoder. xz_dec_lzma2_reset() must be used
// before calling xz_dec_lzma2_run().
//
// Decode the LZMA2 properties (one byte) and reset the decoder. Return
// XZ_OK on success, XZ_MEMLIMIT_ERROR if the preallocated dictionary is not
// big enough, and XZ_OPTIONS_ERROR if props indicates something that this
// decoder doesn't support.
//
extern "C" {
    pub fn xz_dec_lzma2_reset(s: *mut xz_dec_lzma2, props: u8) -> xz_ret;
}
// Decode raw LZMA2 stream from b->in to b->out.
extern "C" {
    pub fn xz_dec_lzma2_run(s: *mut xz_dec_lzma2, b: *mut xz_buf) -> xz_ret;
}
// Free the memory allocated for the LZMA2 decoder.
extern "C" {
    pub fn xz_dec_lzma2_end(s: *mut xz_dec_lzma2);
}

//
// Allocate memory for BCJ decoders. xz_dec_bcj_reset() must be used before
// calling xz_dec_bcj_run().
//
// Decode the Filter ID of a BCJ filter. This implementation doesn't
// support custom start offsets, so no decoding of Filter Properties
// is needed. Returns XZ_OK if the given Filter ID is supported.
// Otherwise XZ_OPTIONS_ERROR is returned.
//
extern "C" {
    pub fn xz_dec_bcj_reset(s: *mut xz_dec_bcj, id: u8) -> xz_ret;
}
//
// Decode raw BCJ + LZMA2 stream. This must be used only if there actually is
// a BCJ filter in the chain. If the chain has only LZMA2, xz_dec_lzma2_run()
// must be called directly.
//
// Free the memory allocated for the BCJ filters.

