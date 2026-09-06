//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/util_mem.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
//
// Generic memory management routines for soundcard memory allocation
//
// memory block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_util_memblk {
    pub /: *mut *mut unsigned int size; / size of this block,
    pub /: *mut *mut unsigned int offset; / zero-offset of this block,
    pub /: *mut *mut list_head list; / link,
}

//
// memory management information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_util_memhdr {
    pub /: *mut *mut unsigned int size; / size of whole data,
    pub /: *mut *mut list_head block; / block linked-list header,
    pub /: *mut *mut int nblocks; / # of allocated blocks,
    pub /: *mut *mut unsigned int used; / used memory size,
    pub /: *mut *mut int block_extra_size; / extra data size of chunk,
    pub /: *mut *mut mutex block_mutex; / lock,
}

//
// prototypes
//
extern "C" {
    pub fn snd_util_memhdr_free(hdr: *mut snd_util_memhdr);
}
extern "C" {
    pub fn snd_util_mem_free(hdr: *mut snd_util_memhdr, blk: *mut snd_util_memblk) -> c_int;
}
extern "C" {
    pub fn snd_util_mem_avail(hdr: *mut snd_util_memhdr) -> c_int;
}
// functions without mutex
extern "C" {
    pub fn __snd_util_mem_free(hdr: *mut snd_util_memhdr, blk: *mut snd_util_memblk);
}
