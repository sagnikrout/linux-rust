//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/dexcr/dexcr.h
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
// POWER Dynamic Execution Control Facility (DEXCR)
//
// This header file contains helper functions and macros
// required for all the DEXCR related test cases.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dexcr_aspect {
    pub /: *const *const *const char name; / Short display name,
    pub /: *const *const *const char opt; / Option name for chdexcr,
    pub /: *const *const *const char desc; / Expanded aspect meaning,
    pub /: *mut *mut unsigned int index; / Aspect bit index in DEXCR,
    pub /: *mut *mut unsigned long prctl; / 'which' value for get/set prctl,
}

extern "C" {
    pub fn dexcr_exists() -> bool;
}
extern "C" {
    pub fn pr_dexcr_aspect_supported(which: c_ulong) -> bool;
}
extern "C" {
    pub fn pr_dexcr_aspect_editable(which: c_ulong) -> bool;
}
extern "C" {
    pub fn pr_get_dexcr(pr_aspect: c_ulong) -> c_int;
}
extern "C" {
    pub fn pr_set_dexcr(pr_aspect: c_ulong, ctrl: c_ulong) -> c_int;
}
extern "C" {
    pub fn pr_which_to_aspect(which: c_ulong) -> c_uint;
}
extern "C" {
    pub fn hashchk_triggers() -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dexcr_source {
    DEXCR,		/* Userspace DEXCR value */
    HDEXCR,		/* Hypervisor enforced DEXCR value */
    EFFECTIVE,	/* Bitwise OR of UDEXCR and ENFORCED DEXCR bits */
}

extern "C" {
    pub fn get_dexcr(source: dexcr_source) -> c_uint;
}
extern "C" {
    pub fn await_child_success(pid: pid_t);
}
extern "C" {
    pub fn hashst(lr: c_ulong, sp: *mut c_void);
}
extern "C" {
    pub fn hashchk(lr: c_ulong, sp: *mut c_void);
}
extern "C" {
    pub fn do_bad_hashchk();
}
