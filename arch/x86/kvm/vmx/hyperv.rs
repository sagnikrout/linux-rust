//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/hyperv.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nested_evmptrld_status {
    EVMPTRLD_DISABLED,
    EVMPTRLD_SUCCEEDED,
    EVMPTRLD_VMFAIL,
    EVMPTRLD_ERROR,
}

extern "C" {
    pub fn evmptr_is_valid(_arg: vmx->nested.hv_evmcs_vmptr) -> return;
}
extern "C" {
    pub fn evmptr_is_set(_arg: vmx->nested.hv_evmcs_vmptr) -> return;
}
//
// eVMCS is exposed to the guest if Hyper-V is enabled in CPUID and
// eVMCS has been explicitly enabled by userspace.
//
extern "C" {
    pub fn nested_get_evmptr(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn nested_get_evmcs_version(vcpu: *mut kvm_vcpu) -> u16;
}
extern "C" {
    pub fn nested_evmcs_filter_control_msr(vcpu: *mut kvm_vcpu, msr_index: u32, pdata: *mut u64);
}
extern "C" {
    pub fn nested_evmcs_check_controls(vmcs12: *mut vmcs12) -> c_int;
}
extern "C" {
    pub fn nested_evmcs_l2_tlb_flush_enabled(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn vmx_hv_inject_synthetic_vmexit_post_tlb_flush(vcpu: *mut kvm_vcpu);
}

