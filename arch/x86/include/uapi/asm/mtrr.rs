//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/mtrr.h
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


// SPDX-License-Identifier: LGPL-2.0+ WITH Linux-syscall-note
// Generic MTRR (Memory Type Range Register) ioctls.
//

// Warning: this structure has a different order from i386

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_sentry {
    pub /: *mut *mut unsigned long base; / Base address,
    pub /: *mut *mut unsigned int size; / Size of region,
    pub /: *mut *mut unsigned int type; / Type of region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_gentry {
    pub /: *mut *mut unsigned int regnum; / Register number,
    pub /: *mut *mut unsigned long base; / Base address,
    pub /: *mut *mut unsigned int size; / Size of region,
    pub /: *mut *mut unsigned int type; / Type of region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_sentry {
    pub /: *mut *mut __u64 base; / Base address,
    pub /: *mut *mut __u32 size; / Size of region,
    pub /: *mut *mut __u32 type; / Type of region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_gentry {
    pub /: *mut *mut __u64 base; / Base address,
    pub /: *mut *mut __u32 size; / Size of region,
    pub /: *mut *mut __u32 regnum; / Register number,
    pub /: *mut *mut __u32 type; / Type of region,
    pub /: *mut *mut __u32 _pad; / Unused,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtrr_var_range {
    pub base_lo: __u32,
    pub base_hi: __u32,
    pub mask_lo: __u32,
    pub mask_hi: __u32,
}

// In the Intel processor's MTRR interface, the MTRR type is always held in
pub type mtrr_type = __u8;
pub const MTRR_NUM_FIXED_RANGES: c_int = 88;
pub const MTRR_MAX_VAR_RANGES: c_int = 256;

// These are the various ioctls

// MTRR memory types, which are defined in SDM
pub const MTRR_TYPE_UNCACHABLE: c_int = 0;
pub const MTRR_TYPE_WRCOMB: c_int = 1;
// #define MTRR_TYPE_         2
// #define MTRR_TYPE_         3
pub const MTRR_TYPE_WRTHROUGH: c_int = 4;
pub const MTRR_TYPE_WRPROT: c_int = 5;
pub const MTRR_TYPE_WRBACK: c_int = 6;
pub const MTRR_NUM_TYPES: c_int = 7;
//
// Invalid MTRR memory type.  No longer used outside of MTRR code.
// Note, this value is allocated from the reserved values (0x7-0xff) of
// the MTRR memory types.
//
pub const MTRR_TYPE_INVALID: c_uint = 0xff;
