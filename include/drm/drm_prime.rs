//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_prime.h
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
// Copyright © 2012 Red Hat
// Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
// Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
// Copyright (c) 2009-2010, Code Aurora Forum.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//
// Authors:
// Dave Airlie <airlied@redhat.com>
// Rob Clark <rob.clark@linaro.org>
//

//
// struct drm_prime_file_private - per-file tracking for PRIME
//
// This just contains the internal &struct dma_buf and handle caches for each
// &struct drm_file used by the PRIME core code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_prime_file_private {
// private:
    pub lock: mutex,
    pub dmabufs: rb_root,
    pub handles: rb_root,
}

// core prime functions
extern "C" {
    pub fn drm_gem_dmabuf_release(dma_buf: *mut dma_buf);
}
// helper functions for exporting
extern "C" {
    pub fn drm_gem_dmabuf_vmap(dma_buf: *mut dma_buf, map: *mut iosys_map) -> c_int;
}
extern "C" {
    pub fn drm_gem_dmabuf_vunmap(dma_buf: *mut dma_buf, map: *mut iosys_map);
}
extern "C" {
    pub fn drm_gem_prime_mmap(obj: *mut drm_gem_object, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn drm_gem_dmabuf_mmap(dma_buf: *mut dma_buf, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn drm_prime_get_contiguous_size(sgt: *mut sg_table) -> c_ulong;
}
// helper functions for importing
extern "C" {
    pub fn drm_prime_gem_destroy(obj: *mut drm_gem_object, sg: *mut sg_table);
}
