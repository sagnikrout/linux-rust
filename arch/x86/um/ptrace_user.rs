//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/ptrace_user.c
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


//
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Licensed under the GPL
//

#[no_mangle]
pub unsafe extern "C" fn ptrace_getregs(pid: c_long, regs_out: *mut c_ulong) -> c_int {
    int ptrace_getregs(long pid, unsigned long *regs_out)
    {
    if (ptrace(PTRACE_GETREGS, pid, 0, regs_out) < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_setregs(pid: c_long, regs: *mut c_ulong) -> c_int {
    int ptrace_setregs(long pid, unsigned long *regs)
    {
    if (ptrace(PTRACE_SETREGS, pid, 0, regs) < 0)
    return -errno;
    return 0;
    }
