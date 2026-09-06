//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/acenv.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// X86 specific ACPICA environments and implementation
//
// Copyright (C) 2014, Intel Corporation
// Author: Lv Zheng <lv.zheng@intel.com>
//

// Asm macros
//
// ACPI_FLUSH_CPU_CACHE() flushes caches on entering sleep states.
// It is required to prevent data loss.
//
// While running inside virtual machine, the kernel can bypass cache flushing.
// Changing sleep state in a virtual machine doesn't affect the host system
// sleep state and cannot lead to data loss.
//

extern "C" {
    pub fn __acpi_acquire_global_lock(lock: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn __acpi_release_global_lock(lock: *mut c_uint) -> c_int;
}

//
// Math helper asm macros
//

