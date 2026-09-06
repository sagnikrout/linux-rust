//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/videobuf2-memops.h
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
// videobuf2-memops.h - generic memory handling routines for videobuf2
//
// Copyright (C) 2010 Samsung Electronics
//
// Author: Pawel Osciak <pawel@osciak.com>
// Marek Szyprowski <m.szyprowski@samsung.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

//
// struct vb2_vmarea_handler - common vma refcount tracking handler.
//
// @refcount:	pointer to &refcount_t entry in the buffer.
// @put:	callback to function that decreases buffer refcount.
// @arg:	argument for @put callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vb2_vmarea_handler {
    pub refcount: *mut refcount_t,
    pub arg): *mut *mut void (put)(void,
    pub arg: *mut c_void,
}

extern "C" {
    pub fn vb2_destroy_framevec(vec: *mut frame_vector);
}
