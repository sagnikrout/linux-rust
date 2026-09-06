//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tracepoint-defs.h
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
pub const TRACEPOINT_DEFS_H: c_int = 1;
//
// File can be included directly by headers who only want to access
// tracepoint->key to guard out of line trace calls, or the definition of
// trace_print_flags{_u64}. Otherwise linux/tracepoint.h should be used.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_print_flags {
    pub mask: c_ulong,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_print_flags_u64 {
    pub mask: c_ulonglong,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracepoint_func {
    pub func: *mut c_void,
    pub data: *mut c_void,
    pub prio: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracepoint_ext {
    pub (*regfunc)(void): *mut c_int,
    pub (*unregfunc)(void): *mut c_void,
// Flags.
    pub faultable:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracepoint {
    pub /: *const *const *const char name; / Tracepoint name,
    pub key: static_key_false,
    pub static_call_key: *mut static_call_key,
    pub static_call_tramp: *mut c_void,
    pub iterator: *mut c_void,
    pub probestub: *mut c_void,
    pub funcs: *mut tracepoint_func __rcu,
    pub ext: *mut tracepoint_ext,
}

pub type tracepoint_ptr_t = c_int;

pub type tracepoint_ptr_t = *mut tracepoint  const;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_raw_event_map {
    pub tp: *mut tracepoint,
    pub bpf_func: *mut c_void,
    pub num_args: u32,
    pub writable_size: u32,
    pub __aligned(32): },
//
// If a tracepoint needs to be called from a header file, it is not
// recommended to call it directly, as tracepoints in header files
// may cause side-effects and bloat the kernel. Instead, use
// tracepoint_enabled() to test if the tracepoint is enabled, then if
// it is, call a wrapper function defined in a C file that will then
// call the tracepoint.
//
// For "trace_foo_bar()", you would need to create a wrapper function
// in a C file to call trace_foo_bar():
// void do_trace_foo_bar(args) { trace_foo_bar(args); }
// Then in the header file, declare the tracepoint:
// DECLARE_TRACEPOINT(foo_bar);
// And call your wrapper:
// static inline void some_inlined_function() {
// [..]
// if (tracepoint_enabled(foo_bar))
// do_trace_foo_bar(args);
// [..]
// }
//
// Note: tracepoint_enabled(foo_bar) is equivalent to trace_foo_bar_enabled()
// but is safe to have in headers, where trace_foo_bar_enabled() is not.
//

