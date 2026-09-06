//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/vmalloc.h
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
// alloc_vmap_area - called when a new vmap allocation occurs
// @addr:	an allocated address
// @size:	a requested size
// @align:	a requested alignment
// @vstart:	a requested start range
// @vend:	a requested end range
// @failed:	an allocation failed or not
//
// This event is used for a debug purpose, it can give an extra
// information for a developer about how often it occurs and which
// parameters are passed for further validation.
//
// purge_vmap_area_lazy - called when vmap areas were lazily freed
// @start:		purging start address
// @end:		purging end address
// @npurged:	numbed of purged vmap areas
//
// This event is used for a debug purpose. It gives some
// indication about start:end range and how many objects
// are released.
//
// free_vmap_area_noflush - called when a vmap area is freed
// @va_start:		a start address of VA
// @nr_lazy:		number of current lazy pages
// @nr_lazy_max:	number of maximum lazy pages
//
// This event is used for a debug purpose. It gives some
// indication about a VA that is released, number of current
// outstanding areas and a maximum allowed threshold before
// dropping all of them.
//

// This part must be outside protection
