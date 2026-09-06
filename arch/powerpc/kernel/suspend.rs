//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/suspend.c
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
// Suspend support specific for power.
//
// Copyright (c) 2002 Pavel Machek <pavel@ucw.cz>
// Copyright (c) 2001 Patrick Mochel <mochel@osdl.org>
//

//
// pfn_is_nosave - check if given pfn is in the 'nosave' section
//
#[no_mangle]
pub unsafe extern "C" fn pfn_is_nosave(pfn: c_ulong) -> c_int {
    int pfn_is_nosave(unsigned long pfn)
    {
    let mut nosave_begin_pfn: c_ulong = __pa(&__nosave_begin) >> PAGE_SHIFT;
    let mut nosave_end_pfn: c_ulong = PAGE_ALIGN(__pa(&__nosave_end)) >> PAGE_SHIFT;
    return (pfn >= nosave_begin_pfn) && (pfn < nosave_end_pfn);
    }
