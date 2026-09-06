//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/seccomp.c
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


// SPDX-License-Identifier: LGPL-2.1

pub const SECCOMP_SET_MODE_STRICT: c_int = 0;

pub const SECCOMP_SET_MODE_FILTER: c_int = 1;

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_seccomp_op(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_seccomp_op(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "SECCOMP_SET_MODE_";
    let mut op: c_int = arg.val;
    let mut printed: usize = 0;
    switch (op) {

    P_SECCOMP_SET_MODE_OP(STRICT);
    P_SECCOMP_SET_MODE_OP(FILTER);

    default: printed = scnprintf(bf, size, "%#x", op);			  break;
    }
    return printed;
    }

pub const SECCOMP_FILTER_FLAG_TSYNC: c_int = 1;

    size_t syscall_arg__scnprintf_seccomp_flags(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "SECCOMP_FILTER_FLAG_";
    let mut printed: c_int = 0, flags = arg.val;

    if (flags & SECCOMP_FILTER_FLAG_##n) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", #n); \
    flags &= ~SECCOMP_FILTER_FLAG_##n; \
    }
    P_FLAG(TSYNC);

    if (flags)
    printed += scnprintf(bf + printed, size - printed, "%s%#x", printed ? "|" : "", flags);
    return printed;
    }
