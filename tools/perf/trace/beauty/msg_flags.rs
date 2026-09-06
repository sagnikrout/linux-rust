//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/msg_flags.c
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

pub const MSG_PROBE: c_uint = 0x10;

pub const MSG_WAITFORONE: c_uint = 0x10000;

pub const MSG_BATCH: c_uint = 0x40000;

pub const MSG_SOCK_DEVMEM: c_uint = 0x2000000;

pub const MSG_ZEROCOPY: c_uint = 0x4000000;

pub const MSG_SPLICE_PAGES: c_uint = 0x8000000;

pub const MSG_FASTOPEN: c_uint = 0x20000000;

    size_t syscall_arg__scnprintf_msg_flags(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "MSG_";
    let mut printed: c_int = 0, flags = arg.val;
    if (flags == 0)
    return scnprintf(bf, size, "NONE");

    if (flags & MSG_##n) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", #n); \
    flags &= ~MSG_##n; \
    }
    P_MSG_FLAG(OOB);
    P_MSG_FLAG(PEEK);
    P_MSG_FLAG(DONTROUTE);
    P_MSG_FLAG(CTRUNC);
    P_MSG_FLAG(PROBE);
    P_MSG_FLAG(TRUNC);
    P_MSG_FLAG(DONTWAIT);
    P_MSG_FLAG(EOR);
    P_MSG_FLAG(WAITALL);
    P_MSG_FLAG(FIN);
    P_MSG_FLAG(SYN);
    P_MSG_FLAG(CONFIRM);
    P_MSG_FLAG(RST);
    P_MSG_FLAG(ERRQUEUE);
    P_MSG_FLAG(NOSIGNAL);
    P_MSG_FLAG(MORE);
    P_MSG_FLAG(WAITFORONE);
    P_MSG_FLAG(BATCH);
    P_MSG_FLAG(SOCK_DEVMEM);
    P_MSG_FLAG(ZEROCOPY);
    P_MSG_FLAG(SPLICE_PAGES);
    P_MSG_FLAG(FASTOPEN);
    P_MSG_FLAG(CMSG_CLOEXEC);

    if (flags)
    printed += scnprintf(bf + printed, size - printed, "%s%#x", printed ? "|" : "", flags);
    return printed;
    }
