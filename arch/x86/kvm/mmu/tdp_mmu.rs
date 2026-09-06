//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/mmu/tdp_mmu.h
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
    pub fn kvm_mmu_init_tdp_mmu(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_mmu_uninit_tdp_mmu(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_tdp_mmu_alloc_root(vcpu: *mut kvm_vcpu, private: bool);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &root->tdp_mmu_root_count) -> return;
}
extern "C" {
    pub fn kvm_tdp_mmu_put_root(kvm: *mut kvm, root: *mut kvm_mmu_page);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_tdp_mmu_root_types {
    KVM_INVALID_ROOTS = BIT(0),
    KVM_DIRECT_ROOTS = BIT(1),
    KVM_MIRROR_ROOTS = BIT(2),
    KVM_VALID_ROOTS = KVM_DIRECT_ROOTS | KVM_MIRROR_ROOTS,
    KVM_ALL_ROOTS = KVM_VALID_ROOTS | KVM_INVALID_ROOTS,
}

extern "C" {
    pub fn root_to_sp(_arg: vcpu->arch.mmu->mirror_root_hpa) -> return;
}
extern "C" {
    pub fn root_to_sp(_arg: vcpu->arch.mmu->root.hpa) -> return;
}
extern "C" {
    pub fn root_to_sp(_arg: vcpu->arch.mmu->mirror_root_hpa) -> return;
}
extern "C" {
    pub fn root_to_sp(_arg: vcpu->arch.mmu->root.hpa) -> return;
}
extern "C" {
    pub fn kvm_tdp_mmu_zap_leafs(kvm: *mut kvm, start: gfn_t, end: gfn_t, flush: bool) -> bool;
}
extern "C" {
    pub fn kvm_tdp_mmu_zap_all(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_tdp_mmu_zap_invalidated_roots(kvm: *mut kvm, shared: bool);
}
extern "C" {
    pub fn kvm_tdp_mmu_map(vcpu: *mut kvm_vcpu, fault: *mut kvm_page_fault) -> c_int;
}
extern "C" {
    pub fn kvm_tdp_mmu_age_gfn_range(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}
extern "C" {
    pub fn kvm_tdp_mmu_test_age_gfn(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}

