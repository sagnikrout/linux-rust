//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/registers.c
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
// Copyright (C) 2004 PathScale, Inc
// Copyright (C) 2004 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

// This is set once at boot time and not changed thereafter
    unsigned long exec_regs[MAX_REG_NR];
    unsigned long *exec_fp_regs;
#[no_mangle]
pub unsafe extern "C" fn init_pid_registers(pid: c_int) -> c_int {
    int init_pid_registers(int pid)
    {
    int err;
    err = ptrace(PTRACE_GETREGS, pid, 0, exec_regs);
    if (err < 0)
    return -errno;
    err = arch_init_registers(pid);
    if (err < 0)
    return err;
    exec_fp_regs = malloc(host_fp_size);
    get_fp_registers(pid, exec_fp_regs);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_safe_registers(regs: *mut c_ulong, fp_regs: *mut c_ulong) {
    void get_safe_registers(unsigned long *regs, unsigned long *fp_regs)
    {
    memcpy(regs, exec_regs, sizeof(exec_regs));
    if (fp_regs)
    memcpy(fp_regs, exec_fp_regs, host_fp_size);
    }
