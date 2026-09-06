//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/user.h
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
// S390 version
//
// Derived from "include/asm-i386/usr.h"
//

// Core file format: The core file is written in such a way that gdb
//
// This is the old layout of "struct pt_regs", and
// is still the layout used by user mode (the new
// pt_regs doesn't have all registers as the kernel
// doesn't use the extra segment registers)
//
// When the kernel dumps core, it starts by dumping the user struct -
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user {
// We start with the registers, to mimic the way that "memory" is returned
    pub /: *mut *mut user_regs_regs; / Where the registers are actually stored,
// The rest of this junk is to help gdb figure out what goes where
    pub /: *mut *mut unsigned long int u_tsize; / Text segment size (pages).,
    pub /: *mut *mut unsigned long int u_dsize; / Data segment size (pages).,
    pub /: *mut *mut unsigned long int u_ssize; / Stack segment size (pages).,
    pub /: *mut *mut unsigned long start_code; / Starting virtual address of text.,
    pub area.: *mut *mut unsigned long start_stack; / Starting virtual address of stack,
    pub /: *mut *mut long int signal; / Signal that caused the core dump.,
    pub /: *mut *mut unsigned long u_ar0; / Used by gdb to help find the values for,
// the registers.
    pub /: *mut *mut unsigned long magic; / To uniquely identify a core file,
    pub /: *mut *mut char u_comm[32]; / User command that was responsible,
}
