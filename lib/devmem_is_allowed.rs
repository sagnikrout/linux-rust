//! Automatically rewritten from C to Rust
//! Source: lib/devmem_is_allowed.c
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
// A generic version of devmem_is_allowed.
//
// Based on arch/arm64/mm/mmap.c
//
// Copyright (C) 2020 Google, Inc.
// Copyright (C) 2012 ARM Ltd.
//

//
// devmem_is_allowed() checks to see if /dev/mem access to a certain address
// is valid. The argument is a physical page number.  We mimic x86 here by
// disallowing access to system RAM as well as device-exclusive MMIO regions.
// This effectively disable read()/write() on /dev/mem.
//
#[no_mangle]
pub unsafe extern "C" fn devmem_is_allowed(pfn: c_ulong) -> c_int {
    int devmem_is_allowed(unsigned long pfn)
    {
    if (iomem_is_exclusive(PFN_PHYS(pfn)))
    return 0;
    if (!page_is_ram(pfn))
    return 1;
    return 0;
    }
