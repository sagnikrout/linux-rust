//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cfi.h
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
// Clang Control Flow Integrity (CFI) support.
//
// Copyright (C) 2022 Google LLC
//

extern "C" {
    pub fn report_cfi_failure(_arg: regs, _arg: addr, _arg: NULL, _arg: 0) -> return;
}

//
// Returns the CFI prefix offset. By default, the compiler emits only
// a 4-byte CFI type hash before the function. If an architecture
// uses -fpatchable-function-entry=N,M where M>0 to change the prefix
// offset, they must override this function.
//

// CFI type hashes for BPF function types

extern "C" {
    pub fn is_cfi_trap(addr: c_ulong) -> bool;
}

// Macro flag: #define CFI_NOSEAL(x)

