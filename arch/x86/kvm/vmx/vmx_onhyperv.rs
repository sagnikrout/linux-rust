//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/vmx_onhyperv.h
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

extern "C" {
    pub fn static_branch_unlikely(_arg: &__kvm_is_using_evmcs) -> return;
}
// (u64 *)((char *)current_evmcs + offset) = value;
// (u32 *)((char *)current_evmcs + offset) = value;
// (u16 *)((char *)current_evmcs + offset) = value;
//
// When enabling eVMCS, KVM verifies that every CPU has a valid hv_vp_assist_page()
// and aborts enabling the feature otherwise. CPU onlining path is also checked in
// vmx_hardware_enable().
//
extern "C" {
    pub fn evmcs_sanitize_exec_ctrls(vmcs_conf: *mut vmcs_config);
}

