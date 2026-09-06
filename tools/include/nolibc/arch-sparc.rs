//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/arch-sparc.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// SPARC (32bit and 64bit) specific definitions for NOLIBC
// Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
//

//
// Syscalls for SPARC:
// - registers are native word size
// - syscall number is passed in g1
// - arguments are in o0-o5
// - the system call is performed by calling a trap instruction
// - syscall return value is in o0
// - syscall error flag is in the carry bit of the processor status register
//

// startup code
//
// Save argc pointer to o0, as arg1 of _start_c.
// Account for the window save area, which is 16 registers wide.
//

extern "C" {
    pub fn getpid() -> static pid_t;
}
// The syscall returns the parent pid in the child instead of 0

// The syscall returns the parent pid in the child instead of 0

