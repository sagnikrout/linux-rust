//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_mksstat.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2021 VMware, Inc., Palo Alto, CA., USA
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// Reservation marker for mksstat pid's

//
// Kernel-internal mksGuestStat counters. The order of this enum dictates the
// order of instantiation of these counters in the mksGuestStat pages.
//
// vmw_mksstat_get_kern_pstat: Computes the address of the MKSGuestStatCounterTime
// array from the address of the base page.
//
// @page_addr: Pointer to the base page.
// Return: Pointer to the MKSGuestStatCounterTime array.
//
// vmw_mksstat_get_kern_pinfo: Computes the address of the MKSGuestStatInfoEntry
// array from the address of the base page.
//
// @page_addr: Pointer to the base page.
// Return: Pointer to the MKSGuestStatInfoEntry array.
//
// vmw_mksstat_get_kern_pstrs: Computes the address of the mksGuestStat strings
// sequence from the address of the base page.
//
// @page_addr: Pointer to the base page.
// Return: Pointer to the mksGuestStat strings sequence.
//
// MKS_STAT_TIME_DECL/PUSH/POP macros to be used in timer-counted routines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mksstat_timer_t {
// mutable */ mksstat_kern_stats_t old_top;
    pub t0: u64,
    pub slot: c_int,
}

// Macro flag: #define MKS_STAT_TIME_DECL(kern_cntr)
// Macro flag: #define MKS_STAT_TIME_PUSH(kern_cntr)
// Macro flag: #define MKS_STAT_TIME_POP(kern_cntr)

