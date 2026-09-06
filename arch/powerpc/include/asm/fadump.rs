//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/fadump.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Firmware Assisted dump header file.
//
// Copyright 2011 IBM Corporation
// Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
//

extern "C" {
    pub fn is_fadump_memory_area(addr: u64, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn setup_fadump() -> c_int;
}
extern "C" {
    pub fn is_fadump_active() -> c_int;
}
extern "C" {
    pub fn should_fadump_crash() -> c_int;
}
extern "C" {
    pub fn crash_fadump(: *mut pt_regs, : *const c_char);
}
extern "C" {
    pub fn fadump_cleanup();
}
extern "C" {
    pub fn fadump_setup_param_area();
}
extern "C" {
    pub fn fadump_append_bootargs();
}

extern "C" {
    pub fn fadump_reserve_mem() -> c_int;
}

extern "C" {
    pub fn fadump_cma_init();
}

