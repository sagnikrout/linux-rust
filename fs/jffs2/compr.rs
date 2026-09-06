//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/compr.h
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


//
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2004   Ferenc Havasi <havasi@inf.u-szeged.hu>,
// University of Szeged, Hungary
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

pub const JFFS2_RUBINMIPS_PRIORITY: c_int = 10;
pub const JFFS2_DYNRUBIN_PRIORITY: c_int = 20;
pub const JFFS2_LZARI_PRIORITY: c_int = 30;
pub const JFFS2_RTIME_PRIORITY: c_int = 50;
pub const JFFS2_ZLIB_PRIORITY: c_int = 60;
pub const JFFS2_LZO_PRIORITY: c_int = 80;

pub const JFFS2_COMPR_MODE_NONE: c_int = 0;
pub const JFFS2_COMPR_MODE_PRIORITY: c_int = 1;
pub const JFFS2_COMPR_MODE_SIZE: c_int = 2;
pub const JFFS2_COMPR_MODE_FAVOURLZO: c_int = 3;
pub const JFFS2_COMPR_MODE_FORCELZO: c_int = 4;
pub const JFFS2_COMPR_MODE_FORCEZLIB: c_int = 5;
pub const FAVOUR_LZO_PERCENT: c_int = 80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_compressor {
    pub list: list_head,
    pub /: *mut *mut int priority; / used by prirority comr. mode,
    pub name: *mut c_char,
    pub /: *mut *mut char compr; / JFFS2_COMPR_XXX,
    pub destlen): *mut *mut uint32_t srclen, uint32_t,
    pub datalen): uint32_t cdatalen, uint32_t,
    pub usecount: c_int,
    pub /: *mut *mut int disabled; / if set the compressor won't compress,
    pub /: *mut *mut *mut unsigned char compr_buf; / used by size compr. mode,
    pub /: *mut *mut uint32_t compr_buf_size; / used by size compr. mode,
    pub stat_compr_orig_size: u32,
    pub stat_compr_new_size: u32,
    pub stat_compr_blocks: u32,
    pub stat_decompr_blocks: u32,
}

extern "C" {
    pub fn jffs2_register_compressor(comp: *mut jffs2_compressor) -> c_int;
}
extern "C" {
    pub fn jffs2_unregister_compressor(comp: *mut jffs2_compressor) -> c_int;
}
extern "C" {
    pub fn jffs2_compressors_init() -> c_int;
}
extern "C" {
    pub fn jffs2_compressors_exit() -> c_int;
}
extern "C" {
    pub fn jffs2_free_comprbuf(comprbuf: *mut c_uchar, orig: *mut c_uchar);
}
// Compressor modules
// These functions will be called by jffs2_compressors_init/exit

extern "C" {
    pub fn jffs2_rubinmips_init() -> c_int;
}
extern "C" {
    pub fn jffs2_rubinmips_exit();
}
extern "C" {
    pub fn jffs2_dynrubin_init() -> c_int;
}
extern "C" {
    pub fn jffs2_dynrubin_exit();
}

extern "C" {
    pub fn jffs2_rtime_init() -> c_int;
}
extern "C" {
    pub fn jffs2_rtime_exit();
}

extern "C" {
    pub fn jffs2_zlib_init() -> c_int;
}
extern "C" {
    pub fn jffs2_zlib_exit();
}

extern "C" {
    pub fn jffs2_lzo_init() -> c_int;
}
extern "C" {
    pub fn jffs2_lzo_exit();
}

