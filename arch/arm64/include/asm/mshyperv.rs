//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/mshyperv.h
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
// Linux-specific definitions for managing interactions with Microsoft's
// Hyper-V hypervisor. The definitions in this file are specific to
// the ARM64 architecture.  See include/asm-generic/mshyperv.h for
// definitions are that architecture independent.
//
// Definitions that are derived from Hyper-V code or headers should not go in
// this file, but should instead go in the relevant files in include/hyperv.
//
// Copyright (C) 2021, Microsoft, Inc.
//
// Author : Michael Kelley <mikelley@microsoft.com>
//

//
// Declare calls to get and set Hyper-V VP register values on ARM64, which
// requires a hypercall.
//
extern "C" {
    pub fn hv_set_vpreg(reg: u32, value: u64);
}
extern "C" {
    pub fn hv_get_vpreg(reg: u32) -> u64;
}
extern "C" {
    pub fn hv_get_vpreg_128(reg: u32, result: *mut hv_get_vp_registers_output);
}
extern "C" {
    pub fn hv_get_vpreg(_arg: reg) -> return;
}
//
// Nested is not supported on arm64
//
extern "C" {
    pub fn hv_get_msr(_arg: reg) -> return;
}
// SMCCC hypercall parameters
pub const HV_SMCCC_FUNC_NUMBER: c_int = 1;

