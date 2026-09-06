//! Automatically rewritten from C Header to Rust Module
//! Source: include/kvm/arm_psci.h
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
// Copyright (C) 2012,2013 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

//
// Our PSCI implementation stays the same across versions from
// v0.2 onward, only adding the few mandatory functions (such
// as FEATURES with 1.0) that are required by newer
// revisions. It is thus safe to return the latest, unless
// userspace has instructed us otherwise.
//
// Narrow the PSCI register arguments (r1 to r3) to 32 bits.
//
// Zero the input registers' upper 32 bits. They will be fully
// zeroed on exit, so we're fine changing them in place.
//
extern "C" {
    pub fn kvm_psci_call(vcpu: *mut kvm_vcpu) -> c_int;
}
