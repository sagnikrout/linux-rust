//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_output.h
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

extern "C" {
    pub fn seq_print_ip_sym(_arg: s, _arg: ip, TRACE_ITER(SYM_OFFSET): sym_flags |) -> return;
}
extern "C" {
    pub fn seq_print_ip_sym(_arg: s, _arg: ip, ~TRACE_ITER(SYM_OFFSET): sym_flags &) -> return;
}
extern "C" {
    pub fn trace_seq_print_sym(s: *mut trace_seq, address: c_ulong, offset: bool);
}
extern "C" {
    pub fn trace_print_context(iter: *mut trace_iterator) -> c_int;
}
extern "C" {
    pub fn trace_print_lat_context(iter: *mut trace_iterator) -> c_int;
}
extern "C" {
    pub fn trace_event_read_lock();
}
extern "C" {
    pub fn trace_event_read_unlock();
}
// used by module unregistering
extern "C" {
    pub fn __unregister_trace_event(event: *mut trace_event) -> c_int;
}

