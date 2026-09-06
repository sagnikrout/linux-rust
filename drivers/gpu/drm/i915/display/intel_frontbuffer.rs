//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_frontbuffer.h
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
// Copyright (c) 2014-2016 Intel Corporation
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fb_op_origin {
    ORIGIN_CPU = 0,
    ORIGIN_CS,
    ORIGIN_FLIP,
    ORIGIN_DIRTYFB,
    ORIGIN_CURSOR_UPDATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_frontbuffer {
    pub display: *mut intel_display,
    pub bits: core::sync::atomic::AtomicI32,
    pub flush_work: work_struct,
}

//
// Frontbuffer tracking bits. Set in obj->frontbuffer_bits while a gem bo is
// considered to be the frontbuffer for the given plane interface-wise. This
// doesn't mean that the hw necessarily already scans it out, but that any
// rendering (by the cpu or gpu) will land in the frontbuffer eventually.
//
// We have one bit per pipe and per scanout plane type.
//
pub const INTEL_FRONTBUFFER_BITS_PER_PIPE: c_int = 8;

//
// intel_frontbuffer_invalidate - invalidate frontbuffer object
// @front: GEM object to invalidate
// @origin: which operation caused the invalidation
//
// This function gets called every time rendering on the given object starts and
// frontbuffer caching (fbc, low refresh rate for DRRS, panel self refresh) must
// be invalidated. For ORIGIN_CS any subsequent invalidation will be delayed
// until the rendering completes or a flip on this frontbuffer plane is
// scheduled.
//
// intel_frontbuffer_flush - flush frontbuffer object
// @front: GEM object to flush
// @origin: which operation caused the flush
//
// This function gets called every time rendering on the given object has
// completed and frontbuffer caching can be started again.
//
extern "C" {
    pub fn intel_frontbuffer_queue_flush(front: *mut intel_frontbuffer);
}
extern "C" {
    pub fn intel_frontbuffer_init(front: *mut intel_frontbuffer, drm: *mut drm_device);
}
extern "C" {
    pub fn intel_frontbuffer_fini(front: *mut intel_frontbuffer);
}
