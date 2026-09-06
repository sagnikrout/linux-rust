//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/arch-powerpc.h
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
// PowerPC specific definitions for NOLIBC
// Copyright (C) 2023 Zhangjin Wu <falcon@tinylab.org>
//

// Syscalls for PowerPC :
// - stack is 16-byte aligned
// - syscall number is passed in r0
// - arguments are in r3, r4, r5, r6, r7, r8, r9
// - the system call is performed by calling "sc"
// - syscall return comes in r3, and the summary overflow bit is checked
// to know if an error occurred, in which case errno is in r3.
// - the arguments are cast to long and assigned into the target
// registers which are then simply passed as registers to the asm code,
// so that we don't have to experience issues with register constraints.
//

// FIXME: For 32-bit PowerPC, with newer gcc compilers (e.g. gcc 13.1.0),
// "omit-frame-pointer" fails with __attribute__((no_stack_protector)) but
// works with __attribute__((__optimize__("-fno-stack-protector")))
//

// startup code

// with -mabi=elfv2, save TOC/GOT pointer to r2
// r12 is global entry pointer, we use it to compute TOC from r12
// https://www.llvm.org/devmtg/2014-04/PDFs/Talks/Euro-LLVM-2014-Weigand.pdf
// https://refspecs.linuxfoundation.org/ELF/ppc64/PPC-elf64abi.pdf
//

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_ftruncate64, _arg: fd, _arg: 0, _arg: length0, _arg: length1) -> return;
}

