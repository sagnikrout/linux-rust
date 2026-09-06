//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kvm/book3s.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright IBM Corporation, 2013
// Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
//
extern "C" {
    pub fn kvm_unmap_gfn_range_hv(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}
extern "C" {
    pub fn kvm_age_gfn_hv(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}
extern "C" {
    pub fn kvm_test_age_gfn_hv(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
}
extern "C" {
    pub fn kvmppc_mmu_init_pr(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_mmu_destroy_pr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_book3s_init_pr() -> c_int;
}
extern "C" {
    pub fn kvmppc_book3s_exit_pr();
}
extern "C" {
    pub fn kvmppc_handle_exit_pr(vcpu: *mut kvm_vcpu, exit_nr: c_uint) -> c_int;
}

extern "C" {
    pub fn kvmppc_emulate_tabort(vcpu: *mut kvm_vcpu, ra_val: c_int);
}

extern "C" {
    pub fn kvmppc_set_msr_hv(vcpu: *mut kvm_vcpu, msr: u64);
}
extern "C" {
    pub fn kvmppc_inject_interrupt_hv(vcpu: *mut kvm_vcpu, vec: c_int, srr1_flags: u64);
}
