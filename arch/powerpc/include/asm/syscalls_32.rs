//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/syscalls_32.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// Data types and macros for providing 32b PowerPC support.
//
// These are here to support 32-bit syscalls on a 64-bit kernel.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs32 {
    pub gpr: [c_uint; 32],
    pub nip: c_uint,
    pub msr: c_uint,
    pub /: *mut *mut unsigned int orig_gpr3; / Used for restarting system calls,
    pub ctr: c_uint,
    pub link: c_uint,
    pub xer: c_uint,
    pub ccr: c_uint,
    pub /: *mut *mut unsigned int mq; / 601 only (not used at present),
    pub /: *mut *mut unsigned int trap; / Reason for being here,
    pub /: *mut *mut unsigned int dar; / Fault registers,
    pub dsisr: c_uint,
    pub /: *mut *mut unsigned int result; / Result of a system call,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext32 {
    pub _unused: [c_uint; 4],
    pub signal: c_int,
    pub handler: compat_uptr_t,
    pub oldmask: c_uint,
    pub /: *mut *mut compat_uptr_t regs; / 4 byte pointer to the pt_regs32 structure.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcontext32 {
    pub mc_gregs: elf_gregset_t32,
    pub mc_fregs: elf_fpregset_t,
    pub mc_pad: [c_uint; 2],
    pub __attribute__((__aligned__(16))): elf_vrregset_t32 mc_vregs,
    pub __attribute__((__aligned__(16))): elf_vsrreghalf_t32 mc_vsregs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucontext32 {
    pub uc_flags: c_uint,
    pub uc_link: c_uint,
    pub uc_stack: compat_stack_t,
    pub uc_pad: [c_int; 7],
    pub /: *mut *mut compat_uptr_t uc_regs; / points to uc_mcontext field,
    pub /: *mut *mut compat_sigset_t uc_sigmask; / mask last for extensibility,
// glibc has 1024-bit signal masks, ours are 64-bit
    pub uc_maskext: [c_int; 30],
    pub uc_pad2: [c_int; 3],
    pub uc_mcontext: mcontext32,
}
