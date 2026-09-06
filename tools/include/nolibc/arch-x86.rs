//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/arch-x86.h
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
// x86 specific definitions for NOLIBC (both 32- and 64-bit)
// Copyright (C) 2017-2025 Willy Tarreau <w@1wt.eu>
//

// Syscalls for i386 :
// - mostly similar to x86_64
// - registers are 32-bit
// - syscall number is passed in eax
// - arguments are in ebx, ecx, edx, esi, edi, ebp respectively
// - all registers are preserved (except eax of course)
// - the system call is performed by calling int $0x80
// - syscall return comes in eax
// - the arguments are cast to long and assigned into the target registers
// which are then simply passed as registers to the asm code, so that we
// don't have to experience issues with register constraints.
// - the syscall number is always specified last in order to allow to force
// some registers before (gcc refuses a %-register at the last position).
//

// startup code
//
// i386 System V ABI mandates:
// 1) last pushed argument must be 16-byte aligned.
// 2) The deepest stack frame should be set to zero
//

// Syscalls for x86_64 :
// - registers are 64-bit
// - syscall number is passed in rax
// - arguments are in rdi, rsi, rdx, r10, r8, r9 respectively
// - the system call is performed by calling the syscall instruction
// - syscall return comes in rax
// - rcx and r11 are clobbered, others are preserved.
// - the arguments are cast to long and assigned into the target registers
// which are then simply passed as registers to the asm code, so that we
// don't have to experience issues with register constraints.
// - the syscall number is always specified last in order to allow to force
// some registers before (gcc refuses a %-register at the last position).
// - see also x86-64 ABI section A.2 AMD64 Linux Kernel Conventions, A.2.1
// Calling Conventions.
//
// Link x86-64 ABI: https://gitlab.com/x86-psABIs/x86-64-ABI/-/wikis/home
//

// startup code
//
// x86-64 System V ABI mandates:
// 1) %rsp must be 16-byte aligned right before the function call.
// 2) The deepest stack frame should be zero (the %rbp).
//

// Macro flag: #define NOLIBC_ARCH_HAS_MEMMOVE
// Macro flag: #define NOLIBC_ARCH_HAS_MEMCPY
// Macro flag: #define NOLIBC_ARCH_HAS_MEMSET

