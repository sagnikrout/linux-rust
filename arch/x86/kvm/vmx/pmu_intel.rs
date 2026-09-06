//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/pmu_intel.h
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

extern "C" {
    pub fn intel_pmu_lbr_is_enabled(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn intel_pmu_create_guest_lbr_event(vcpu: *mut kvm_vcpu) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbr_desc {
// Basic info about guest LBR records.
    pub records: x86_pmu_lbr,
//
// Emulate LBR feature via passthrough LBR registers when the
// per-vcpu guest LBR event is scheduled on the current pcpu.
//
// The records may be inaccurate if the host reclaims the LBR.
//
    pub event: *mut perf_event,
// True if LBRs are marked as not intercepted in the MSR bitmap
    pub msr_passthrough: bool,
}
