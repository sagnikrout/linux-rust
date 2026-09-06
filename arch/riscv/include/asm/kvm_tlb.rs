//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_tlb.h
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
// Copyright (c) 2025 Ventana Micro Systems Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_riscv_hfence_type {
    KVM_RISCV_HFENCE_UNKNOWN = 0,
    KVM_RISCV_HFENCE_GVMA_VMID_GPA,
    KVM_RISCV_HFENCE_GVMA_VMID_ALL,
    KVM_RISCV_HFENCE_VVMA_ASID_GVA,
    KVM_RISCV_HFENCE_VVMA_ASID_ALL,
    KVM_RISCV_HFENCE_VVMA_GVA,
    KVM_RISCV_HFENCE_VVMA_ALL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_riscv_hfence {
    pub type: kvm_riscv_hfence_type,
    pub asid: c_ulong,
    pub vmid: c_ulong,
    pub order: c_ulong,
    pub addr: gpa_t,
    pub size: gpa_t,
}

pub const KVM_RISCV_VCPU_MAX_HFENCE: c_int = 64;
pub const KVM_RISCV_GSTAGE_TLB_MIN_ORDER: c_int = 12;
extern "C" {
    pub fn kvm_riscv_local_hfence_gvma_vmid_all(vmid: c_ulong);
}
extern "C" {
    pub fn kvm_riscv_local_hfence_gvma_all();
}
extern "C" {
    pub fn kvm_riscv_local_hfence_vvma_all(vmid: c_ulong);
}
extern "C" {
    pub fn kvm_riscv_local_tlb_sanitize(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_tlb_flush_process(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_fence_i_process(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_hfence_vvma_all_process(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_hfence_process(vcpu: *mut kvm_vcpu);
}
