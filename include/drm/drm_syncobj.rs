//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_syncobj.h
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
// Copyright © 2017 Red Hat
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
//

//
// struct drm_syncobj - sync object.
//
// This structure defines a generic sync object which wraps a &dma_fence.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj {
//
// @refcount: Reference count of this object.
//
    pub refcount: kref,
//
// @fence:
// NULL or a pointer to the fence bound to this object.
//
// This field should not be used directly. Use drm_syncobj_fence_get()
// and drm_syncobj_replace_fence() instead.
//
    pub fence: *mut dma_fence __rcu,
//
// @cb_list: List of callbacks to call when the &fence gets replaced.
//
    pub cb_list: list_head,
//
// @ev_fd_list: List of registered eventfd.
//
    pub ev_fd_list: list_head,
//
// @lock: Protects &cb_list and &ev_fd_list, and write-locks &fence.
//
    pub lock: spinlock_t,
//
// @file: A file backing for this syncobj.
//
    pub file: *mut file,
}

extern "C" {
    pub fn drm_syncobj_free(kref: *mut kref);
}
//
// drm_syncobj_get - acquire a syncobj reference
// @obj: sync object
//
// This acquires an additional reference to @obj. It is illegal to call this
// without already holding a reference. No locks required.
//
// drm_syncobj_put - release a reference to a sync object.
// @obj: sync object.
//
// drm_syncobj_fence_get - get a reference to a fence in a sync object
// @syncobj: sync object.
//
// This acquires additional reference to &drm_syncobj.fence contained in @obj,
// if not NULL. It is illegal to call this without already holding a reference.
// No locks required.
//
// Returns:
// Either the fence of @obj or NULL if there's none.
//
extern "C" {
    pub fn drm_syncobj_free(kref: *mut kref);
}
extern "C" {
    pub fn drm_syncobj_get_fd(syncobj: *mut drm_syncobj, p_fd: *mut c_int) -> c_int;
}
