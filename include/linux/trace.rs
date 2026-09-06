//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/trace.h
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
// The trace export - an export of Ftrace output. The trace_export
// can process traces and export them to a registered destination as
// an addition to the current only output of Ftrace - i.e. ring buffer.
//
// If you want traces to be sent to some other place rather than ring
// buffer only, just need to register a new trace_export and implement
// its own .write() function for writing traces to the storage.
//
// next		- pointer to the next trace_export
// write	- copy traces which have been delt with ->commit() to
// the destination
// flags	- which ftrace to be exported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_export {
    pub next: *mut trace_export __rcu,
    pub int): *const *const *const *const void (write)(struct trace_export , void , unsigned,
    pub flags: c_int,
}

extern "C" {
    pub fn register_ftrace_export(export: *mut trace_export) -> c_int;
}
extern "C" {
    pub fn unregister_ftrace_export(export: *mut trace_export) -> c_int;
}
//
// trace_array_puts - write a constant string into the trace buffer.
// @tr:    The trace array to write to
// @str:   The constant string to write
//

extern "C" {
    pub fn trace_printk_init_buffers();
}
extern "C" {
    pub fn trace_array_init_printk(tr: *mut trace_array) -> c_int;
}
extern "C" {
    pub fn trace_array_put(tr: *mut trace_array);
}
extern "C" {
    pub fn trace_array_destroy(tr: *mut trace_array) -> c_int;
}
// For osnoise tracer
extern "C" {
    pub fn osnoise_arch_register() -> c_int;
}
extern "C" {
    pub fn osnoise_arch_unregister();
}
extern "C" {
    pub fn osnoise_trace_irq_entry(id: c_int);
}
extern "C" {
    pub fn osnoise_trace_irq_exit(id: c_int, desc: *const c_char);
}

