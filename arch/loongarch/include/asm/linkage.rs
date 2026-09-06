//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/linkage.h
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
// This is for the signal handler trampoline, which is used as the return
// address of the signal handlers in userspace instead of called normally.
// The long standing libgcc bug https://gcc.gnu.org/PR124050 requires a
// nop between .cfi_startproc and the actual address of the trampoline, so
// we cannot simply use SYM_FUNC_START.
//
// This wrapper also contains all the .cfi_* directives for recovering
// the content of the GPRs and the "return address" (where the rt_sigreturn
// syscall will jump to), assuming there is a struct rt_sigframe (where
// a struct sigcontext containing those information we need to recover) at
// $sp.  The "DWARF for the LoongArch(TM) Architecture" manual states
// column 0 is for $zero, but it does not make too much sense to
// save/restore the hardware zero register.  Repurpose this column here
// for the return address (here it's not the content of $ra we cannot use
// the default column 3).
//

