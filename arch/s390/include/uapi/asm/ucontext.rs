//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/ucontext.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// S390 version
//
// Derived from "include/asm-i386/ucontext.h"
//

//
// The struct ucontext_extended describes how the registers are stored
// on a rt signal frame. Please note that the structure is not fixed,
// if new CPU registers are added to the user state the size of the
// struct ucontext_extended will increase.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucontext_extended {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: _sigregs,
    pub uc_sigmask: sigset_t,
// Allow for uc_sigmask growth.  Glibc uses a 1024-bit sigset_t.
    pub sizeof(sigset_t)]: unsigned char __unused[128 -,
    pub uc_mcontext_ext: _sigregs_ext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucontext {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: _sigregs,
    pub uc_sigmask: sigset_t,
// Allow for uc_sigmask growth.  Glibc uses a 1024-bit sigset_t.
    pub sizeof(sigset_t)]: unsigned char __unused[128 -,
}
