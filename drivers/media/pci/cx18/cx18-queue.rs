//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-queue.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cx18 buffer queues
//
// Derived from ivtv-queue.h
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//

// cx18_buffer utility functions
extern "C" {
    pub fn _cx18_mdl_sync_for_device(s: *mut cx18_stream, mdl: *mut cx18_mdl);
}
extern "C" {
    pub fn cx18_buf_swap(buf: *mut cx18_buffer);
}
extern "C" {
    pub fn _cx18_mdl_swap(mdl: *mut cx18_mdl);
}
// cx18_queue utility functions
extern "C" {
    pub fn cx18_queue_init(q: *mut cx18_queue);
}
extern "C" {
    pub fn cx18_flush_queues(s: *mut cx18_stream);
}
// queue MDL reconfiguration helpers
extern "C" {
    pub fn cx18_unload_queues(s: *mut cx18_stream);
}
extern "C" {
    pub fn cx18_load_queues(s: *mut cx18_stream);
}
// cx18_stream utility functions
extern "C" {
    pub fn cx18_stream_alloc(s: *mut cx18_stream) -> c_int;
}
extern "C" {
    pub fn cx18_stream_free(s: *mut cx18_stream);
}
