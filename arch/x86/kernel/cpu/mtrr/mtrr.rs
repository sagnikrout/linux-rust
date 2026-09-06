//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/mtrr/mtrr.h
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
// local MTRR defines.
//

pub const MTRR_CHANGE_MASK_FIXED: c_uint = 0x01;
pub const MTRR_CHANGE_MASK_VARIABLE: c_uint = 0x02;
pub const MTRR_CHANGE_MASK_DEFTYPE: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_ops {
    pub var_regs: u32,
    pub type): unsigned long size, mtrr_type,
    pub type): *mut *mut unsigned long size, mtrr_type,
    pub replace_reg): c_int,
    pub type): c_uint,
    pub (*have_wrcomb)(void): *mut c_int,
}

extern "C" {
    pub fn positive_have_wrcomb() -> c_int;
}
// library functions for processor-specific routines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_mtrr_context {
    pub flags: c_ulong,
    pub cr4val: c_ulong,
    pub deftype_lo: u32,
    pub deftype_hi: u32,
    pub ccr3: u32,
}

extern "C" {
    pub fn get_mtrr_state() -> bool;
}
extern "C" {
    pub fn mtrr_state_warn();
}
extern "C" {
    pub fn mtrr_wrmsr(_arg: unsigned, _arg: unsigned, _arg: unsigned);
}

extern "C" {
    pub fn mtrr_set_if();
}
extern "C" {
    pub fn mtrr_register_syscore();
}

extern "C" {
    pub fn mtrr_build_map();
}
extern "C" {
    pub fn mtrr_copy_map();
}
// CPU specific mtrr_ops vectors.
extern "C" {
    pub fn mtrr_cleanup() -> c_int;
}
//
// Must be used by code which uses mtrr_if to call platform-specific
// MTRR manipulation functions.
//
extern "C" {
    pub fn generic_rebuild_map();
}
