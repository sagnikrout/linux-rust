//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/uio.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Berkeley style UIO structures	-	Alan Cox 1994.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmabuf_cmsg {
    pub starts.: *mut *mut __u64 frag_offset; / offset into the dmabuf where the frag,
//
    pub /: *mut *mut __u32 frag_size; / size of the frag.,
    pub for: *mut *mut __u32 frag_token; / token representing this frag,
// DEVMEM_DONTNEED.
//
    pub /: *mut *mut __u32 dmabuf_id; / dmabuf id this frag belongs to.,
    pub future: *mut *mut __u32 flags; / Currently unused. Reserved for,
// uses.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmabuf_token {
    pub token_start: __u32,
    pub token_count: __u32,
}

//
// UIO_MAXIOV shall be at least 16 1003.1g (5.4.1.1)
//
pub const UIO_FASTIOV: c_int = 8;
pub const UIO_MAXIOV: c_int = 1024;
