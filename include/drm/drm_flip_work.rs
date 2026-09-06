//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_flip_work.h
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
// Copyright (C) 2013 Red Hat
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
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// DOC: flip utils
//
// Utility to queue up work to run from work-queue context after flip/vblank.
// Typically this can be used to defer unref of framebuffer's, cursor
// bo's, etc until after vblank. The APIs are all thread-safe. Moreover,
// drm_flip_work_commit() can be called in atomic context.
//
// drm_flip_func_t - callback function
//
// @work: the flip work
// @val: value queued via drm_flip_work_queue()
//
// Callback function to be called for each of the  queue'd work items after
// drm_flip_work_commit() is called.
//
extern "C" {
    pub fn void(work: *mut *mut drm_flip_func_t)(struct drm_flip_work, val: *mut c_void) -> typedef;
}
//
// struct drm_flip_work - flip work queue
// @name: debug name
// @func: callback fxn called for each committed item
// @worker: worker which calls @func
// @queued: queued tasks
// @commited: commited tasks
// @lock: lock to access queued and commited lists
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_flip_work {
    pub name: *const c_char,
    pub func: drm_flip_func_t,
    pub worker: work_struct,
    pub queued: list_head,
    pub commited: list_head,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn drm_flip_work_queue(work: *mut drm_flip_work, val: *mut c_void);
}
extern "C" {
    pub fn drm_flip_work_cleanup(work: *mut drm_flip_work);
}
