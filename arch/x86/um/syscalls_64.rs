//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/syscalls_64.c
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
// Copyright (C) 2003 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Copyright 2003 PathScale, Inc.
//
// Licensed under the GPL
//

    long arch_prctl(struct task_struct *task, int option,
    unsigned long __user *arg2)
    {
    let mut ret: c_long = -EINVAL;
    switch (option) {
    case ARCH_SET_FS:
    current.thread.regs.regs.gp[FS_BASE / sizeof(unsigned long)] =
    (unsigned long) arg2;
    ret = 0;
    break;
    case ARCH_SET_GS:
    current.thread.regs.regs.gp[GS_BASE / sizeof(unsigned long)] =
    (unsigned long) arg2;
    ret = 0;
    break;
    case ARCH_GET_FS:
    ret = put_user(current.thread.regs.regs.gp[FS_BASE / sizeof(unsigned long)], arg2);
    break;
    case ARCH_GET_GS:
    ret = put_user(current.thread.regs.regs.gp[GS_BASE / sizeof(unsigned long)], arg2);
    break;
    }
    return ret;
    }
    SYSCALL_DEFINE2(arch_prctl, int, option, unsigned long, arg2)
    {
    return arch_prctl(current, option, (unsigned long __user *) arg2);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_switch_to(to: *mut task_struct) {
    void arch_switch_to(struct task_struct *to)
    {
//
// Nothing needs to be done on x86_64.
// The FS_BASE/GS_BASE registers are saved in the ptrace register set.
//
    }
    SYSCALL_DEFINE6(mmap, unsigned long, addr, unsigned long, len,
    unsigned long, prot, unsigned long, flags,
    unsigned long, fd, unsigned long, off)
    {
    if (off & ~PAGE_MASK)
    return -EINVAL;
    return ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> PAGE_SHIFT);
    }
