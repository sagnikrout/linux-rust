//! Automatically rewritten from C Header to Rust Module
//! Source: fs/erofs/compress.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2019 HUAWEI, Inc.
// https://www.huawei.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_decompress_req {
    pub sb: *mut super_block,
    pub out: *mut *mut *mut page in,,
    pub outpages: unsigned int inpages,,
    pub pageofs_out: unsigned short pageofs_in,,
    pub outputsize: unsigned int inputsize,,
    pub /: *mut *mut unsigned int alg; / the algorithm for decompression,
    pub fillgaps: bool inplace_io, partial_decoding,,
    pub /: *mut *mut gfp_t gfp; / allocation flags for extra temporary buffers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_decompressor {
    pub size): *mut *mut void data, int,
    pub pagepool): *mut page,
    pub (*init)(void): *mut c_int,
    pub (*exit)(void): *mut c_void,
    pub name: *mut c_char,
}

//
// Currently, short-lived pages are pages directly from buddy system
// with specific page->private (Z_EROFS_SHORTLIVED_PAGE).
// In the future world of Memdescs, it should be type 0 (Misc) memory
// which type can be checked with a new helper.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_stream_dctx {
    pub rq: *mut z_erofs_decompress_req,
    pub /: *mut *mut int no, ni; / the current {en,de}coded page #,
    pub /: *mut *mut unsigned int avail_out; / remaining bytes in the decoded buffer,
    pub inbuf_sz: unsigned int inbuf_pos,,
// current status of the encoded buffer
    pub /: *mut *mut *mut *mut u8 kin, kout; / buffer mapped pointers,
    pub /: *mut *mut *mut void bounce; / bounce buffer for inplace I/Os,
    pub /: *mut *mut bool bounced; / is the bounce buffer used now?,
}

extern "C" {
    pub fn z_erofs_init_decompressor() -> int __init;
}
extern "C" {
    pub fn z_erofs_exit_decompressor();
}
extern "C" {
    pub fn z_erofs_crypto_enable_engine(name: *const c_char, len: c_int) -> c_int;
}

extern "C" {
    pub fn z_erofs_crypto_disable_all_engines();
}
extern "C" {
    pub fn z_erofs_crypto_show_engines(buf: *mut c_char, size: c_int, sep: c_char) -> c_int;
}

