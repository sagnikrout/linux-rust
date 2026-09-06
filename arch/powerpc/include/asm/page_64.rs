//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/page_64.h
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
// Copyright (C) 2001 PPC64 Team, IBM Corp
//

//
// We always define HW_PAGE_SHIFT to 12 as use of 64K pages remains Linux
// specific, every notion of page number shared with the firmware, TCEs,
// iommu, etc... still uses a page size of 4K.
//
pub const HW_PAGE_SHIFT: c_int = 12;

//
// PAGE_FACTOR is the number of bits factor between PAGE_SHIFT and
// HW_PAGE_SHIFT, that is 4K pages.
//

// Segment size; normal 256M segments
pub const SID_SHIFT: c_int = 28;

pub const ESID_MASK: c_uint = 0xfffffffff0000000UL;

// 1T segments
pub const SID_SHIFT_1T: c_int = 40;
pub const SID_MASK_1T: c_uint = 0xffffffUL;
pub const ESID_MASK_1T: c_uint = 0xffffff0000000000UL;

pub type pte_basic_t = c_ulong;
//
// Some verisions of gcc use multiply instructions to
// calculate the offsets so lets give it a hand to
// do better.
//
extern "C" {
    pub fn copy_page(to: *mut c_void, from: *mut c_void);
}
// Log 2 of page table size

//
// This is the default if a program doesn't have a PT_GNU_STACK
// program header entry. The PPC64 ELF ABI has a non executable stack
// stack by default, so in the absence of a PT_GNU_STACK program header
// we turn execute permission off.
//

