//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mtrr.h
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


// SPDX-License-Identifier: LGPL-2.0+
// Generic MTRR (Memory Type Range Register) ioctls.
//

// Defines for hardware MTRR registers.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_state_type {
    pub var_ranges: [mtrr_var_range; MTRR_MAX_VAR_RANGES],
    pub fixed_ranges: [mtrr_type; MTRR_NUM_FIXED_RANGES],
    pub enabled: c_uchar,
    pub have_fixed: bool,
    pub def_type: mtrr_type,
}

//
// The following functions are for use by other drivers that cannot use
// arch_phys_wc_add and arch_phys_wc_del.
//

extern "C" {
    pub fn mtrr_bp_init();
}
extern "C" {
    pub fn mtrr_type_lookup(addr: u64, end: u64, uniform: *mut u8) -> u8;
}
extern "C" {
    pub fn mtrr_save_fixed_ranges(: *mut c_void);
}
extern "C" {
    pub fn mtrr_save_state();
}
extern "C" {
    pub fn mtrr_del(reg: c_int, base: c_ulong, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn mtrr_del_page(reg: c_int, base: c_ulong, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn mtrr_trim_uncached_memory(end_pfn: c_ulong) -> c_int;
}
extern "C" {
    pub fn amd_special_default_mtrr() -> c_int;
}
extern "C" {
    pub fn mtrr_disable();
}
extern "C" {
    pub fn mtrr_enable();
}
extern "C" {
    pub fn mtrr_generic_set_state();
}

//
// Return the default MTRR type, without any known other types in
// that range.
//
// uniform = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_sentry32 {
    pub /: *mut *mut compat_ulong_t base; / Base address,
    pub /: *mut *mut compat_uint_t size; / Size of region,
    pub /: *mut *mut compat_uint_t type; / Type of region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_gentry32 {
    pub /: *mut *mut compat_ulong_t regnum; / Register number,
    pub /: *mut *mut compat_uint_t base; / Base address,
    pub /: *mut *mut compat_uint_t size; / Size of region,
    pub /: *mut *mut compat_uint_t type; / Type of region,
}

// Bit fields for enabled in struct mtrr_state_type
pub const MTRR_STATE_SHIFT: c_int = 10;

