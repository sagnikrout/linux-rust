//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/sigframe.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigframe_ia32 {
    pub pretcode: u32,
    pub sig: c_int,
    pub sc: sigcontext_32,
//
// fpstate is unused. fpstate is moved/allocated after
// retcode[] below. This movement allows to have the FP state and the
// future state extensions (xsave) stay together.
// And at the same time retaining the unused fpstate, prevents changing
// the offset of extramask[] in the sigframe and thus prevent any
// legacy application accessing/modifying it.
//
    pub fpstate_unused: _fpstate_32,
    pub extramask: [c_uint; 1],
    pub retcode: [c_char; 8],
// fp state follows here
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_sigframe_ia32 {
    pub pretcode: u32,
    pub sig: c_int,
    pub pinfo: u32,
    pub puc: u32,

    pub info: compat_siginfo_t,

    pub info: siginfo,

    pub uc: ucontext_ia32,
    pub retcode: [c_char; 8],
// fp state follows here
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_sigframe {
    pub pretcode: *mut char __user,
    pub uc: ucontext,
    pub info: siginfo,
// fp state follows here
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucontext_x32 {
    pub uc_flags: c_uint,
    pub uc_link: c_uint,
    pub uc_stack: compat_stack_t,
    pub /: *mut *mut unsigned int uc__pad0; / needed for alignment,
    pub /: *mut *mut sigcontext uc_mcontext; / the 64-bit sigcontext type,
    pub /: *mut *mut compat_sigset_t uc_sigmask; / mask last for extensibility,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_sigframe_x32 {
    pub pretcode: u64,
    pub uc: ucontext_x32,
    pub info: compat_siginfo_t,
// fp state follows here
}

