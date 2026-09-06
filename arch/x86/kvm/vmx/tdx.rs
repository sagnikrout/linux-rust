//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/tdx.h
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
    pub fn tdx_hardware_setup() -> c_int;
}
extern "C" {
    pub fn tdx_hardware_unsetup();
}
// TDX module hardware states. These follow the TDX module OP_STATEs.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_tdx_state {
    TD_STATE_UNINITIALIZED = 0,
    TD_STATE_INITIALIZED,
    TD_STATE_RUNNABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_tdx {
    pub kvm: kvm,
    pub misc_cg: *mut misc_cg,
    pub hkid: c_int,
    pub state: kvm_tdx_state,
    pub attributes: u64,
    pub xfam: u64,
    pub tsc_offset: u64,
    pub tsc_multiplier: u64,
    pub td: tdx_td,
//
// Scratch pointer used to pass the source page to tdx_mem_page_add().
// Protected by slots_lock, and non-NULL only when mapping a private
// pfn via tdx_gmem_post_populate().
//
    pub page_add_src: *mut page,
//
// Prevent vCPUs from TD entry to ensure SEPT zap related SEAMCALLs do
// not contend with tdh_vp_enter() and TDCALLs.
// Set/unset is protected with kvm->mmu_lock.
//
    pub wait_for_sept_zap: bool,
}

// TDX module vCPU states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcpu_tdx_state {
    VCPU_TD_STATE_UNINITIALIZED = 0,
    VCPU_TD_STATE_INITIALIZED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_tdx {
    pub vcpu: kvm_vcpu,
    pub vt: vcpu_vt,
    pub ext_exit_qualification: u64,
    pub exit_gpa: gpa_t,
    pub vp_enter_args: tdx_module_args,
    pub vp: tdx_vp,
    pub cpu_list: list_head,
    pub vp_enter_ret: u64,
    pub state: vcpu_tdx_state,
    pub map_gpa_next: u64,
    pub map_gpa_end: u64,
}

extern "C" {
    pub fn tdh_vp_rd_failed(tdx: *mut vcpu_tdx, uclass: *mut c_char, field: u32, err: u64);
}
pub const VMCS_ENC_ACCESS_TYPE_MASK: c_uint = 0x1UL;
pub const VMCS_ENC_ACCESS_TYPE_FULL: c_uint = 0x0UL;
pub const VMCS_ENC_ACCESS_TYPE_HIGH: c_uint = 0x1UL;

// TDX is 64bit only.  HIGH field isn't supported.

// TDX is 64bit only.  i.e. natural width = 64bit.

extern "C" {
    pub fn tdx_interrupt_allowed(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn tdx_complete_emulated_msr(vcpu: *mut kvm_vcpu, err: c_int) -> c_int;
}

pub const enable_tdx: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_tdx {
    pub kvm: kvm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_tdx {
    pub vcpu: kvm_vcpu,
}

