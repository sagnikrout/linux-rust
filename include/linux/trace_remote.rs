//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/trace_remote.h
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
// struct trace_remote_callbacks - Callbacks used by Tracefs to control the remote
// @init:		Called once the remote has been registered. Allows the
// caller to extend the Tracefs remote directory
// @load_trace_buffer:  Called before Tracefs accesses the trace buffer for the first
// time. Must return a &trace_buffer_desc
// (most likely filled with trace_remote_alloc_buffer())
// @unload_trace_buffer:
// Called once Tracefs has no use for the trace buffer
// (most likely call trace_remote_free_buffer())
// @enable_tracing:	Called on Tracefs tracing_on. It is expected from the
// remote to allow writing.
// @swap_reader_page:	Called when Tracefs consumes a new page from a
// ring-buffer. It is expected from the remote to isolate a
// @reset:		Called on `echo 0 > trace`. It is expected from the
// remote to reset all ring-buffer pages.
// new reader-page from the @cpu ring-buffer.
// @enable_event:	Called on events/event_name/enable. It is expected from
// the remote to allow the writing event @id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_remote_callbacks {
    pub priv): *mut *mut *mut int (init)(struct dentry d, void,
    pub priv): *mut *mut *mut trace_buffer_desc (load_trace_buffer)(unsigned long size, void,
    pub priv): *mut *mut *mut void (unload_trace_buffer)(struct trace_buffer_desc desc, void,
    pub priv): *mut *mut int (enable_tracing)(bool enable, void,
    pub priv): *mut *mut int (swap_reader_page)(unsigned int cpu, void,
    pub priv): *mut *mut int (reset)(unsigned int cpu, void,
    pub priv): *mut *mut int (enable_event)(unsigned short id, bool enable, void,
}

extern "C" {
    pub fn trace_remote_free_buffer(desc: *mut trace_buffer_desc);
}
