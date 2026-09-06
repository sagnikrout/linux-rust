//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/signal32.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 ARM Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_sigcontext {
// We always set these two fields to 0
    pub trap_no: compat_ulong_t,
    pub error_code: compat_ulong_t,
    pub oldmask: compat_ulong_t,
    pub arm_r0: compat_ulong_t,
    pub arm_r1: compat_ulong_t,
    pub arm_r2: compat_ulong_t,
    pub arm_r3: compat_ulong_t,
    pub arm_r4: compat_ulong_t,
    pub arm_r5: compat_ulong_t,
    pub arm_r6: compat_ulong_t,
    pub arm_r7: compat_ulong_t,
    pub arm_r8: compat_ulong_t,
    pub arm_r9: compat_ulong_t,
    pub arm_r10: compat_ulong_t,
    pub arm_fp: compat_ulong_t,
    pub arm_ip: compat_ulong_t,
    pub arm_sp: compat_ulong_t,
    pub arm_lr: compat_ulong_t,
    pub arm_pc: compat_ulong_t,
    pub arm_cpsr: compat_ulong_t,
    pub fault_address: compat_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ucontext {
    pub uc_flags: compat_ulong_t,
    pub uc_link: compat_uptr_t,
    pub uc_stack: compat_stack_t,
    pub uc_mcontext: compat_sigcontext,
    pub uc_sigmask: compat_sigset_t,
    pub sizeof(int))]: int __unused[32 - (sizeof(compat_sigset_t) /,
    pub __attribute__((__aligned__(8))): compat_ulong_t uc_regspace[128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_sigframe {
    pub uc: compat_ucontext,
    pub retcode: [compat_ulong_t; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_rt_sigframe {
    pub info: compat_siginfo,
    pub sig: compat_sigframe,
}

extern "C" {
    pub fn compat_setup_restart_syscall(regs: *mut pt_regs);
}

