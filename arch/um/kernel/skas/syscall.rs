//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/skas/syscall.c
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[no_mangle]
pub unsafe extern "C" fn handle_syscall(r: *mut uml_pt_regs) {
    void handle_syscall(struct uml_pt_regs *r)
    {
    struct pt_regs *regs = container_of(r, struct pt_regs, regs);
    int syscall;
// Initialize the syscall number and default return value.
    UPT_SYSCALL_NR(r) = PT_SYSCALL_NR(r.gp);
    PT_REGS_SET_SYSCALL_RETURN(regs, -ENOSYS);
    if (syscall_trace_enter(regs))
    goto out;
// Do the seccomp check after ptrace; failures should be fast.
    if (!seccomp_permit_syscall())
    goto out;
    syscall = UPT_SYSCALL_NR(r);
//
// If no time passes, then sched_yield may not actually yield, causing
// broken spinlock implementations in userspace (ASAN) to hang for long
// periods of time.
//
    if ((time_travel_mode == TT_MODE_INFCPU ||
    time_travel_mode == TT_MODE_EXTERNAL) &&
    syscall == __NR_sched_yield)
    tt_extra_sched_jiffies += 1;
    if (syscall >= 0 && syscall < __NR_syscalls) {
    unsigned long ret;
    ret = (*sys_call_table[syscall])(UPT_SYSCALL_ARG1(&regs.regs),
    UPT_SYSCALL_ARG2(&regs.regs),
    UPT_SYSCALL_ARG3(&regs.regs),
    UPT_SYSCALL_ARG4(&regs.regs),
    UPT_SYSCALL_ARG5(&regs.regs),
    UPT_SYSCALL_ARG6(&regs.regs));
    PT_REGS_SET_SYSCALL_RETURN(regs, ret);
//
// An error value here can be some form of -ERESTARTSYS
// and then we'd just loop. Make any error syscalls take
// some time, so that it won't just loop if something is
// not ready, and hopefully other things will make some
// progress.
//
    if (IS_ERR_VALUE(ret) &&
    (time_travel_mode == TT_MODE_INFCPU ||
    time_travel_mode == TT_MODE_EXTERNAL)) {
    um_udelay(1);
    schedule();
    }
    }
    out:
    syscall_trace_leave(regs);
    }
