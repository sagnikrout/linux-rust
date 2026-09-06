//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/mem-reservation.h
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
// Xen memory reservation utilities.
//
// Copyright (c) 2003, B Dragovic
// Copyright (c) 2003-2004, M Williamson, K Fraser
// Copyright (c) 2005 Dan M. Smith, IBM Corporation
// Copyright (c) 2010 Daniel Kiper
// Copyright (c) 2018 Oleksandr Andrushchenko, EPAM Systems Inc.
//

extern "C" {
    pub fn xenmem_reservation_increase(count: c_int, frames: *mut xen_pfn_t) -> c_int;
}
extern "C" {
    pub fn xenmem_reservation_decrease(count: c_int, frames: *mut xen_pfn_t) -> c_int;
}
