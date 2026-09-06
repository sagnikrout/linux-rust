//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/hv-24x7-domains.h
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
// DOMAIN(name, num, index_kind, is_physical)
//
// @name:	An all caps token, suitable for use in generating an enum
// member and appending to an event name in sysfs.
//
// @num:	The number corresponding to the domain as given in
// documentation. We assume the catalog domain and the hcall
// domain have the same numbering (so far they do), but this
// may need to be changed in the future.
//
// @index_kind: A stringifiable token describing the meaning of the index
// within the given domain. Must fit the parsing rules of the
// perf sysfs api.
//
// @is_physical: True if the domain is physical, false otherwise (if virtual).
//
// Note: The terms PHYS_CHIP, PHYS_CORE, VCPU correspond to physical chip,
// physical core and virtual processor in 24x7 Counters specifications.
//
