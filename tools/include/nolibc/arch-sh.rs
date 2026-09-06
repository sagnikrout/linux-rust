//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/arch-sh.h
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
// SuperH specific definitions for NOLIBC
// Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
//

//
// Syscalls for SuperH:
// - registers are 32bit wide
// - syscall number is passed in r3
// - arguments are in r4, r5, r6, r7, r0, r1, r2
// - the system call is performed by calling trapa #31
// - syscall return value is in r0
//

// startup code
extern "C" {
    pub fn _start_wrapper();
}

