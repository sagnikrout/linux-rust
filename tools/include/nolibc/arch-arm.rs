//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/arch-arm.h
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
// ARM specific definitions for NOLIBC
// Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
//

// Syscalls for ARM in ARM or Thumb modes :
// - registers are 32-bit
// - stack is 8-byte aligned
// ( http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.faqs/ka4127.html)
// - syscall number is passed in r7
// - arguments are in r0, r1, r2, r3, r4, r5
// - the system call is performed by calling svc #0
// - syscall return comes in r0.
// - only lr is clobbered.
// - the arguments are cast to long and assigned into the target registers
// which are then simply passed as registers to the asm code, so that we
// don't have to experience issues with register constraints.
// - the syscall number is always specified last in order to allow to force
// some registers before (gcc refuses a %-register at the last position).
// - in thumb mode without -fomit-frame-pointer, r7 is also used to store the
// frame pointer, and we cannot directly assign it as a register variable,
// nor can we clobber it. Instead we assign the r6 register and swap it
// with r7 before calling svc, and r6 is marked as clobbered.
// We're just using any regular register which we assign to r7 after saving
// it.
//

// swap r6,r7 needed in Thumb mode since we can't use nor clobber r7

// in Arm mode we can directly use r7

// startup code

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_ftruncate64, _arg: fd, _arg: 0, _arg: length0, _arg: length1) -> return;
}

