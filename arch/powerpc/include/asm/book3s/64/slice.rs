//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/slice.h
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

// Macro flag: #define HAVE_ARCH_HUGETLB_UNMAPPED_AREA

// Macro flag: #define HAVE_ARCH_UNMAPPED_AREA
// Macro flag: #define HAVE_ARCH_UNMAPPED_AREA_TOPDOWN

pub const SLICE_LOW_SHIFT: c_int = 28;

pub const SLICE_HIGH_SHIFT: c_int = 40;

extern "C" {
    pub fn get_slice_psize(mm: *mut mm_struct, addr: c_ulong) -> c_uint;
}
extern "C" {
    pub fn slice_init_new_context_exec(mm: *mut mm_struct);
}
extern "C" {
    pub fn slice_setup_new_exec();
}

