//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kvm/timing.h
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
// Copyright IBM Corp. 2008
//
// Authors: Christian Ehrhardt <ehrhardt@linux.vnet.ibm.com>
//

extern "C" {
    pub fn kvmppc_init_timing_stats(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_update_timing_stats(vcpu: *mut kvm_vcpu);
}

// if exit timing is not configured there is no need to build the c file

// account the exit in kvm_stats
// type has to be known at build time for optimization
// wrapper to set exit time and account for it in kvm_stats
