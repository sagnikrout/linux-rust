//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/arch-mips.h
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
// MIPS specific definitions for NOLIBC
// Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
//

// Syscalls for MIPS ABI O32 :
// - WARNING! there's always a delayed slot!
// - WARNING again, the syntax is different, registers take a '$' and numbers
// do not.
// - registers are 32-bit
// - stack is 8-byte aligned
// - syscall number is passed in v0 (starts at 0xfa0).
// - arguments are in a0, a1, a2, a3, then the stack. The caller needs to
// leave some room in the stack for the callee to save a0..a3 if needed.
// - Many registers are clobbered, in fact only a0..a2 and s0..s8 are
// preserved. See: https://www.linux-mips.org/wiki/Syscall as well as
// scall32-o32.S in the kernel sources.
// - the system call is performed by calling "syscall"
// - syscall return comes in v0, and register a3 needs to be checked to know
// if an error occurred, in which case errno is in v0.
// - the arguments are cast to long and assigned into the target registers
// which are then simply passed as registers to the asm code, so that we
// don't have to experience issues with register constraints.
//
// Syscalls for MIPS ABI N32, same as ABI O32 with the following differences :
// - arguments are in a0, a1, a2, a3, t0, t1, t2, t3.
// t0..t3 are also known as a4..a7.
// - stack is 16-byte aligned
//

// binutils, GCC and clang disagree about register aliases, use numbers instead.

// startup code, note that it's called __start on MIPS
extern "C" {
    pub fn __start();
}

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_ftruncate64, _arg: fd, _arg: 0, _arg: length0, _arg: length1) -> return;
}

