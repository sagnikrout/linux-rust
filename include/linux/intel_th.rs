//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/intel_th.h
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
// Intel(R) Trace Hub data structures for implementing buffer sinks.
//
// Copyright (C) 2019 Intel Corporation.
//

// MSC operating modes (MSC_MODE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msu_buffer {
    pub name: *const c_char,
//
// ->assign() called when buffer 'mode' is set to this driver
// (aka mode_store())
// @device:	struct device * of the msc
// @mode:	allows the driver to set HW mode (see the enum above)
// Returns:	a pointer to a private structure associated with this
// msc or NULL in case of error. This private structure
// will then be passed into all other callbacks.
//
    pub mode): *mut *mut *mut *mut void (assign)(struct device dev, int,
// ->unassign():	some other mode is selected, clean up
    pub priv): *mut *mut void (unassign)(void,
//
// ->alloc_window(): allocate memory for the window of a given
// size
// @sgt:	pointer to sg_table, can be overridden by the buffer
// driver, or kept intact
// Returns:	number of sg table entries <= number of pages;
// 0 is treated as an allocation failure.
//
    pub size): usize,
    pub sgt): *mut *mut *mut void (free_window)(void priv, struct sg_table,
// ->activate():	trace has started
    pub priv): *mut *mut void (activate)(void,
// ->deactivate():	trace is about to stop
    pub priv): *mut *mut void (deactivate)(void,
//
// ->ready():	window @sgt is filled up to the last block OR
// tracing is stopped by the user; this window contains
// @bytes data. The window in question transitions into
// the "LOCKED" state, indicating that it can't be used
// by hardware. To clear this state and make the window
// available to the hardware again, call
// intel_th_msc_window_unlock().
//
    pub bytes): *mut *mut *mut *mut int (ready)(void priv, struct sg_table sgt, size_t,
}

extern "C" {
    pub fn intel_th_msu_buffer_unregister(mbuf: *const msu_buffer);
}
extern "C" {
    pub fn intel_th_msc_window_unlock(dev: *mut device, sgt: *mut sg_table);
}

