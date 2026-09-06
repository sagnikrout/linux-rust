//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_vblank_work.h
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


// SPDX-License-Identifier: MIT

//
// struct drm_vblank_work - A delayed work item which delays until a target
// vblank passes, and then executes at realtime priority outside of IRQ
// context.
//
// See also:
// drm_vblank_work_schedule()
// drm_vblank_work_init()
// drm_vblank_work_cancel_sync()
// drm_vblank_work_flush()
// drm_vblank_work_flush_all()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vblank_work {
//
// @base: The base &kthread_work item which will be executed by
// &drm_vblank_crtc.worker. Drivers should not interact with this
// directly, and instead rely on drm_vblank_work_init() to initialize
// this.
//
    pub base: kthread_work,
//
// @vblank: A pointer to &drm_vblank_crtc this work item belongs to.
//
    pub vblank: *mut drm_vblank_crtc,
//
// @count: The target vblank this work will execute on. Drivers should
// not modify this value directly, and instead use
// drm_vblank_work_schedule()
//
    pub count: u64,
//
// @cancelling: The number of drm_vblank_work_cancel_sync() calls that
// are currently running. A work item cannot be rescheduled until all
// calls have finished.
//
    pub cancelling: c_int,
//
// @node: The position of this work item in
// &drm_vblank_crtc.pending_work.
//
    pub node: list_head,
}

//
// to_drm_vblank_work - Retrieve the respective &drm_vblank_work item from a
// &kthread_work
// @_work: The &kthread_work embedded inside a &drm_vblank_work
//

extern "C" {
    pub fn drm_vblank_work_cancel_sync(work: *mut drm_vblank_work) -> bool;
}
extern "C" {
    pub fn drm_vblank_work_flush(work: *mut drm_vblank_work);
}
extern "C" {
    pub fn drm_vblank_work_flush_all(crtc: *mut drm_crtc);
}
