//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/ftrace.h
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
//
// arch/arm64/include/asm/ftrace.h
//
// Copyright (C) 2013 Linaro Limited
// Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
//

// Macro flag: #define HAVE_FUNCTION_GRAPH_FP_TEST

pub const ARCH_SUPPORTS_FTRACE_OPS: c_int = 1;

// The BL at the callsite's adjusted rec->ip

pub const FTRACE_PLT_IDX: c_int = 0;
pub const NR_FTRACE_PLTS: c_int = 1;
//
// Currently, gcc tends to save the link register after the local variables
// on the stack. This causes the max stack tracer to report the function
// frame sizes for the wrong functions. By defining
// ARCH_FTRACE_SHIFT_STACK_TRACER, it will tell the stack tracer to expect
// to find the return address on the stack after the local variables have
// been set up.
//
// Note, this may change in the future, and we will need to deal with that
// if it were to happen.
//
pub const ARCH_FTRACE_SHIFT_STACK_TRACER: c_int = 1;

extern "C" {
    pub fn _mcount(long: unsigned);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_arch_ftrace {
// No extra data needed for arm64
}

extern "C" {
    pub fn return_to_handler();
}
extern "C" {
    pub fn ftrace_call_adjust(addr: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn arch_ftrace_get_symaddr(fentry_ip: c_ulong) -> c_ulong;
}

// Macro flag: #define HAVE_ARCH_FTRACE_REGS

//
// Note: sizeof(struct ftrace_regs) must be a multiple of 16 to ensure correct
// stack alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __arch_ftrace_regs {
// x0 - x8
    pub regs: [c_ulong; 9],
    pub direct_tramp: c_ulong,

    pub __unused: c_ulong,

    pub fp: c_ulong,
    pub lr: c_ulong,
    pub sp: c_ulong,
    pub pc: c_ulong,
}

extern "C" {
    pub fn ftrace_regs_query_register_offset(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn ftrace_init_nop(mod: *mut module, rec: *mut dyn_ftrace) -> c_int;
}

//
// The ftrace trampoline will return to this address instead of the
// instrumented function.
//

//
// Because AArch32 mode does not share the same syscall table with AArch64,
// tracing compat syscalls may result in reporting bogus syscalls or even
// hang-up, so just do not trace them.
// See kernel/trace/trace_syscalls.c
//
// x86 code says:
// If the user really wants these, then they should use the
// raw syscall tracepoints with filtering.
//
// Macro flag: #define ARCH_TRACE_IGNORE_COMPAT_SYSCALLS
extern "C" {
    pub fn is_compat_task() -> return;
}
// Macro flag: #define ARCH_HAS_SYSCALL_MATCH_SYM_NAME
//
// Since all syscall functions have __arm64_ prefix, we must skip it.
// However, as we described above, we decided to ignore compat
// syscalls, so we don't care about __arm64_compat_ prefix here.
//

