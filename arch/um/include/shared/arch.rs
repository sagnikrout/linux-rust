//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/shared/arch.h
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
// Copyright (C) 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

extern "C" {
    pub fn arch_check_bugs();
}
extern "C" {
    pub fn arch_fixup(address: c_ulong, regs: *mut uml_pt_regs) -> c_int;
}
extern "C" {
    pub fn arch_examine_signal(sig: c_int, regs: *mut uml_pt_regs);
}
extern "C" {
    pub fn mc_set_rip(_mc: *mut c_void, target: *mut c_void);
}
