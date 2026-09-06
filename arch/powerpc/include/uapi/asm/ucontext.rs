//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/ucontext.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcontext {
    pub mc_gregs: elf_gregset_t,
    pub mc_fregs: elf_fpregset_t,
    pub mc_pad: [c_ulong; 2],
    pub __attribute__((__aligned__(16))): elf_vrregset_t mc_vregs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucontext {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext __user,
    pub uc_stack: stack_t,
    pub uc_pad: [c_int; 7],
    pub /: *mut *mut *mut mcontext __user uc_regs;/ points to uc_mcontext field,

    pub uc_sigmask: sigset_t,
// glibc has 1024-bit signal masks, ours are 64-bit

    pub /: *mut *mut sigset_t __unused[15]; / Allow for uc_sigmask growth,
    pub /: *mut *mut sigcontext uc_mcontext; / last for extensibility,
    pub uc_maskext: [c_int; 30],
    pub uc_pad2: [c_int; 3],
    pub uc_mcontext: mcontext,

}
