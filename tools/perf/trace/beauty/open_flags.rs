//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/open_flags.c
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

pub const O_DIRECT: c_int = 00040000;

pub const O_DIRECTORY: c_int = 00200000;

pub const O_NOATIME: c_int = 01000000;

pub const O_TMPFILE: c_int = 020000000;

pub const O_LARGEFILE: c_int = 00100000;
#[no_mangle]
pub unsafe extern "C" fn open__scnprintf_flags(flags: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    size_t open__scnprintf_flags(unsigned long flags, char *bf, size_t size, bool show_prefix)
    {
    const char *prefix = "O_";
    let mut printed: c_int = 0;
    if ((flags & O_ACCMODE) == O_RDONLY)
    printed = scnprintf(bf, size, "%s%s", show_prefix ? prefix : "", "RDONLY");
    if (flags == 0)
    return printed;

    if (flags & O_##n) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", #n); \
    flags &= ~O_##n; \
    }
    P_FLAG(RDWR);
    P_FLAG(APPEND);
    P_FLAG(ASYNC);
    P_FLAG(CLOEXEC);
    P_FLAG(CREAT);
    P_FLAG(DIRECT);
    P_FLAG(DIRECTORY);
    P_FLAG(EXCL);
    P_FLAG(LARGEFILE);
    P_FLAG(NOFOLLOW);
    P_FLAG(TMPFILE);
    P_FLAG(NOATIME);
    P_FLAG(NOCTTY);

    P_FLAG(NONBLOCK);

    P_FLAG(NDELAY);

    P_FLAG(PATH);

    if ((flags & O_SYNC) == O_SYNC)
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", "SYNC");
    else {
    P_FLAG(DSYNC);
    }

    P_FLAG(SYNC);

    P_FLAG(TRUNC);
    P_FLAG(WRONLY);

    if (flags)
    printed += scnprintf(bf + printed, size - printed, "%s%#x", printed ? "|" : "", flags);
    return printed;
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_open_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_open_flags(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut flags: c_int = arg.val;
    if (!(flags & O_CREAT))
    arg.mask |= 1 << (arg.idx + 1); /* Mask the mode parm */
    return open__scnprintf_flags(flags, bf, size, arg.show_string_prefix);
    }
