//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/socket_type.c
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

pub const SOCK_TYPE_MASK: c_uint = 0xf;

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_socket_type(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_socket_type(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "SOCK_";
    size_t printed;
    int type = arg.val,
    flags = type & ~SOCK_TYPE_MASK;
    type &= SOCK_TYPE_MASK;
//
// Can't use a strarray, MIPS may override for ABI reasons.
//
    switch (type) {

    P_SK_TYPE(STREAM);
    P_SK_TYPE(DGRAM);
    P_SK_TYPE(RAW);
    P_SK_TYPE(RDM);
    P_SK_TYPE(SEQPACKET);
    P_SK_TYPE(DCCP);
    P_SK_TYPE(PACKET);

    default:
    printed = scnprintf(bf, size, "%#x", type);
    }

    if (flags & SOCK_##n) { \
    printed += scnprintf(bf + printed, size - printed, "|%s", #n); \
    flags &= ~SOCK_##n; \
    }
    P_SK_FLAG(CLOEXEC);
    P_SK_FLAG(NONBLOCK);

    if (flags)
    printed += scnprintf(bf + printed, size - printed, "|%#x", flags);
    return printed;
    }
