//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/flock.c
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

pub const LOCK_MAND: c_int = 32;

pub const LOCK_READ: c_int = 64;

pub const LOCK_WRITE: c_int = 128;

pub const LOCK_RW: c_int = 192;

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_flock(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_flock(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "LOCK_";
    let mut printed: c_int = 0, op = arg.val;
    if (op == 0)
    return scnprintf(bf, size, "NONE");

    if ((op & LOCK_##cmd) == LOCK_##cmd) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", #cmd); \
    op &= ~LOCK_##cmd; \
    }
    P_CMD(SH);
    P_CMD(EX);
    P_CMD(NB);
    P_CMD(UN);
    P_CMD(MAND);
    P_CMD(RW);
    P_CMD(READ);
    P_CMD(WRITE);

    if (op)
    printed += scnprintf(bf + printed, size - printed, "%s%#x", printed ? "|" : "", op);
    return printed;
    }
