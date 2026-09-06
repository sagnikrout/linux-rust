//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/raw3270.h
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


// SPDX-License-Identifier: GPL-2.0
//
// IBM/3270 Driver
//
// Author(s):
// Original 3270 Code for 2.4 written by Richard Hitt (UTS Global)
// Rewritten for 2.5 by Martin Schwidefsky <schwidefsky@de.ibm.com>
// Copyright IBM Corp. 2003, 2009
//

// 3270 CCW request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw3270_request {
    pub /: *mut *mut list_head list; / list head for request queueing.,
    pub /: *mut *mut *mut raw3270_view view; / view of this request,
    pub /: *mut *mut ccw1 ccw; / single ccw.,
    pub /: *mut *mut *mut void buffer; / output buffer.,
    pub /: *mut *mut size_t size; / size of output buffer.,
    pub /: *mut *mut int rescnt; / residual count from devstat.,
    pub /: *mut *mut int rc; / return code for this request.,
// Callback for delivering final status.
    pub data): *mut *mut *mut void (callback)(struct raw3270_request rq, void,
    pub callback_data: *mut c_void,
}

extern "C" {
    pub fn raw3270_request_free(rq: *mut raw3270_request);
}
extern "C" {
    pub fn raw3270_request_reset(rq: *mut raw3270_request) -> c_int;
}
extern "C" {
    pub fn raw3270_request_set_cmd(rq: *mut raw3270_request, cmd: u8);
}
extern "C" {
    pub fn raw3270_request_add_data(rq: *mut raw3270_request, data: *mut c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn raw3270_request_set_data(rq: *mut raw3270_request, data: *mut c_void, size: usize);
}
extern "C" {
    pub fn raw3270_request_set_idal(rq: *mut raw3270_request, ib: *mut idal_buffer);
}
extern "C" {
    pub fn list_empty(_arg: &rq->list) -> return;
}
extern "C" {
    pub fn raw3270_buffer_address(: *mut raw3270, : *mut c_char, _arg: c_int, _arg: c_int);
}
//
// Functions of a 3270 view.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw3270_fn {
    pub rq): *mut *mut int (activate)(struct raw3270_view,
    pub rq): *mut *mut void (deactivate)(struct raw3270_view,
    pub ib): *mut *mut raw3270_request rq, irb,
    pub view): *mut *mut void (release)(struct raw3270_view,
    pub view): *mut *mut void (free)(struct raw3270_view,
    pub old_rows): int old_model, int old_cols, int,
}

//
// View structure chaining. The raw3270_view structure is meant to
// be embedded at the start of the real view data structure, e.g.:
// struct example {
// struct raw3270_view view;
// ...
// };
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw3270_view {
    pub list: list_head,
    pub /: *mut *mut spinlock_t lock; / protects members of view,
pub const RAW3270_VIEW_LOCK_IRQ: c_int = 0;
pub const RAW3270_VIEW_LOCK_BH: c_int = 1;
    pub ref_count: core::sync::atomic::AtomicI32,
    pub dev: *mut raw3270,
    pub fn: *mut raw3270_fn,
    pub model: c_uint,
    pub /: *mut *mut unsigned int rows, cols; / # of rows & colums of the view,
    pub /: *mut *mut *mut unsigned char ascebc; / ascii -> ebcdic table,
}

extern "C" {
    pub fn raw3270_add_view(view: *mut raw3270_view, fn: *mut raw3270_fn, minor: c_int, subclass: c_int) -> c_int;
}
extern "C" {
    pub fn raw3270_view_lock_unavailable(view: *mut raw3270_view) -> c_int;
}
extern "C" {
    pub fn raw3270_activate_view(view: *mut raw3270_view) -> c_int;
}
extern "C" {
    pub fn raw3270_del_view(view: *mut raw3270_view);
}
extern "C" {
    pub fn raw3270_deactivate_view(view: *mut raw3270_view);
}
extern "C" {
    pub fn raw3270_start(view: *mut raw3270_view, rq: *mut raw3270_request) -> c_int;
}
extern "C" {
    pub fn raw3270_start_locked(view: *mut raw3270_view, rq: *mut raw3270_request) -> c_int;
}
extern "C" {
    pub fn raw3270_start_irq(view: *mut raw3270_view, rq: *mut raw3270_request) -> c_int;
}
extern "C" {
    pub fn raw3270_reset(view: *mut raw3270_view) -> c_int;
}
extern "C" {
    pub fn raw3270_view_active(view: *mut raw3270_view) -> c_int;
}
extern "C" {
    pub fn raw3270_read_modified_cb(rq: *mut raw3270_request, data: *mut c_void);
}
// Reference count inliner for view structures.
extern "C" {
    pub fn raw3270_wait_cons_dev(rp: *mut raw3270);
}
// Notifier for device addition/removal
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw3270_notifier {
    pub list: list_head,
    pub minor): *mut *mut void (create)(int,
    pub minor): *mut *mut void (destroy)(int,
}

extern "C" {
    pub fn raw3270_register_notifier(notifier: *mut raw3270_notifier) -> c_int;
}
extern "C" {
    pub fn raw3270_unregister_notifier(notifier: *mut raw3270_notifier);
}
