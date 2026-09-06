//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/capabilities.h
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

pub const PT_MODE_SYSTEM: c_int = 0;
pub const PT_MODE_HOST_GUEST: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_vmx_msrs {
//
// We only store the "true" versions of the VMX capability MSRs. We
// generate the "non-true" versions by setting the must-be-1 bits
// according to the SDM.
//
    pub procbased_ctls_low: u32,
    pub procbased_ctls_high: u32,
    pub secondary_ctls_low: u32,
    pub secondary_ctls_high: u32,
    pub pinbased_ctls_low: u32,
    pub pinbased_ctls_high: u32,
    pub exit_ctls_low: u32,
    pub exit_ctls_high: u32,
    pub entry_ctls_low: u32,
    pub entry_ctls_high: u32,
    pub misc_low: u32,
    pub misc_high: u32,
    pub ept_caps: u32,
    pub vpid_caps: u32,
    pub basic: u64,
    pub cr0_fixed0: u64,
    pub cr0_fixed1: u64,
    pub cr4_fixed0: u64,
    pub cr4_fixed1: u64,
    pub vmcs_enum: u64,
    pub vmfunc_controls: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcs_config {
    pub basic: u64,
    pub pin_based_exec_ctrl: u32,
    pub cpu_based_exec_ctrl: u32,
    pub cpu_based_2nd_exec_ctrl: u32,
    pub cpu_based_3rd_exec_ctrl: u64,
    pub vmexit_ctrl: u32,
    pub vmentry_ctrl: u32,
    pub misc: u64,
    pub nested: nested_vmx_msrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmx_capability {
    pub ept: u32,
    pub vpid: u32,
}

extern "C" {
    pub fn cpu_has_vmx_tpr_shadow(lapic_in_kernel(vcpu: ) &&) -> return;
}
// check if the cpu supports writing r/o exit information fields
//
// Processor Trace can operate in one of three modes:
// a. system-wide: trace both host/guest and output to host buffer
// b. host-only:   only trace host and output to host buffer
// c. host-guest:  trace host and guest simultaneously and output to their
// respective buffer
//
// KVM currently only supports (a) and (c).
//
