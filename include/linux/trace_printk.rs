//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/trace_printk.h
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
// General tracing related utility functions - trace_printk(),
// tracing_on/tracing_off and tracing_start()/tracing_stop
//
// Use tracing_on/tracing_off when you want to quickly turn on or off
// tracing. It simply enables or disables the recording of the trace events.
// This also corresponds to the user space /sys/kernel/tracing/tracing_on
// file, which gives a means for the kernel and userspace to interact.
// Place a tracing_off() in the kernel where you want tracing to end.
// From user space, examine the trace, and then echo 1 > tracing_on
// to continue tracing.
//
// tracing_stop/tracing_start has slightly more overhead. It is used
// by things like suspend to ram where disabling the recording of the
// trace is not enough, but tracing must actually stop because things
// like calling smp_processor_id() may crash the system.
//
// Most likely, you want to use tracing_on/tracing_off.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ftrace_dump_mode {
    DUMP_NONE,
    DUMP_ALL,
    DUMP_ORIG,
    DUMP_PARAM,
}

extern "C" {
    pub fn tracing_on();
}
extern "C" {
    pub fn tracing_off();
}
extern "C" {
    pub fn tracing_is_on() -> c_int;
}
extern "C" {
    pub fn tracing_snapshot();
}
extern "C" {
    pub fn tracing_snapshot_alloc();
}
extern "C" {
    pub fn tracing_start();
}
extern "C" {
    pub fn tracing_stop();
}

//
// trace_printk - printf formatting in the ftrace buffer
// @fmt: the printf format for printing
//
// Note: __trace_printk is an internal function for trace_printk() and
// the @ip is passed in via the trace_printk() macro.
//
// This function allows a kernel developer to debug fast path sections
// that printk is not appropriate for. By scattering in various
// printk like tracing in the code, a developer can quickly see
// where problems are occurring.
//
// This is intended as a debugging tool for the developer only.
// Please refrain from leaving trace_printks scattered around in
// your code. (Extra memory is used for special buffers that are
// allocated when trace_printk() is used.)
//
// A little optimization trick is done here. If there's only one
// argument, there's no need to scan the string for printf formats.
// The trace_puts() will suffice. But how can we take advantage of
// using trace_puts() when trace_printk() has only one argument?
// By stringifying the args and checking the size we can tell
// whether or not there are args. __stringify((__VA_ARGS__)) will
// turn into "()\0" with a size of 3 when there are no args, anything
// else will be bigger. All we need to do is define a string to this,
// and then take its size and compare to 3. If it's bigger, use
// do_trace_printk() otherwise, optimize it to trace_puts(). Then just
// let gcc optimize the rest.
//

extern "C" {
    pub fn __trace_bprintk(ip: c_ulong, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn __trace_printk(ip: c_ulong, fmt: *const c_char, ...) -> c_int;
}
//
// trace_puts - write a string into the ftrace buffer
// @str: the string to record
//
// Note: __trace_bputs is an internal function for trace_puts and
// the @ip is passed in via the trace_puts macro.
//
// This is similar to trace_printk() but is made for those really fast
// paths that a developer wants the least amount of "Heisenbug" effects,
// where the processing of the print format is still too much.
//
// This function allows a kernel developer to debug fast path sections
// that printk is not appropriate for. By scattering in various
// printk like tracing in the code, a developer can quickly see
// where problems are occurring.
//
// This is intended as a debugging tool for the developer only.
// Please refrain from leaving trace_puts scattered around in
// your code. (Extra memory is used for special buffers that are
// allocated when trace_puts() is used.)
//
// Returns: 0 if nothing was written, positive # if string was.
// (1 when __trace_bputs is used, strlen(str) when __trace_puts is used)
//

extern "C" {
    pub fn __trace_bputs(ip: c_ulong, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn __trace_puts(ip: c_ulong, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn trace_dump_stack(skip: c_int);
}
//
// The double __builtin_constant_p is because gcc will give us an error
// if we try to allocate the static variable to fmt if it is not a
// constant. Even with the outer if statement.
//

extern "C" {
    pub fn ftrace_dump(oops_dump_mode: ftrace_dump_mode);
}

