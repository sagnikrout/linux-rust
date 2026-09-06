//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/arm/page.h
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

// Xen machine address
// Xen pseudo-physical address

//
// The pseudo-physical frame (pfn) used in all the helpers is always based
// on Xen page granularity (i.e 4KB).
//
// A Linux page may be split across multiple non-contiguous Xen page so we
// have to keep track with frame based on 4KB page granularity.
//
// PV drivers should never make a direct usage of those helpers (particularly
// pfn_to_gfn and gfn_to_pfn).
//
extern "C" {
    pub fn __pfn_to_mfn(pfn: c_ulong) -> c_ulong;
}
// Pseudo-physical <-> Guest conversion
// Pseudo-physical <-> BUS conversion

// VIRT <-> GUEST conversion

// Only used in PV code. But ARM guests are always HVM.
extern "C" {
    pub fn __set_phys_to_machine(pfn: c_ulong, mfn: c_ulong) -> bool;
}
extern "C" {
    pub fn __set_phys_to_machine(_arg: pfn, _arg: mfn) -> return;
}
