//! Automatically rewritten from C to Rust
//! Source: lib/compat_audit.c
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

    unsigned int compat_dir_class[] = {

    ~0U
    };
    unsigned int compat_read_class[] = {

    ~0U
    };
    unsigned int compat_write_class[] = {

    ~0U
    };
    unsigned int compat_chattr_class[] = {

    ~0U
    };
    unsigned int compat_signal_class[] = {

    ~0U
    };
#[no_mangle]
pub unsafe extern "C" fn audit_classify_compat_syscall(abi: c_int, syscall: c_uint) -> c_int {
    int audit_classify_compat_syscall(int abi, unsigned int syscall)
    {
    switch (syscall) {

    case __NR_open:
    return AUDITSC_OPEN;

    case __NR_openat:
    return AUDITSC_OPENAT;

    case __NR_socketcall:
    return AUDITSC_SOCKETCALL;

    case __NR_execve:
    return AUDITSC_EXECVE;

    case __NR_openat2:
    return AUDITSC_OPENAT2;

    default:
    return AUDITSC_COMPAT;
    }
    }
