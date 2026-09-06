//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/trigger_consumer.h
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


// SPDX-License-Identifier: GPL-2.0-only
// The industrial I/O core, trigger consumer functions
//
// Copyright (c) 2008-2011 Jonathan Cameron
//

//
// struct iio_poll_func - poll function pair
//
// @indio_dev:			data specific to device (passed into poll func)
// @h:				the function that is actually run on trigger
// @thread:			threaded interrupt part
// @type:			the type of interrupt (basically if oneshot)
// @name:			name used to identify the trigger consumer.
// @irq:			the corresponding irq as allocated from the
// trigger pool
// @timestamp:			some devices need a timestamp grabbed as soon
// as possible after the trigger - hence handler
// passes it via here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_poll_func {
    pub indio_dev: *mut iio_dev,
    pub p): *mut *mut irqreturn_t (h)(int irq, void,
    pub p): *mut *mut irqreturn_t (thread)(int irq, void,
    pub type: c_int,
    pub name: *mut c_char,
    pub irq: c_int,
    pub timestamp: i64,
}

// iio_alloc_pollfunc(irqreturn_t (*h)(int irq, void *p),
extern "C" {
    pub fn iio_dealloc_pollfunc(pf: *mut iio_poll_func);
}
extern "C" {
    pub fn iio_pollfunc_store_time(irq: c_int, p: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn iio_trigger_notify_done(trig: *mut iio_trigger);
}
