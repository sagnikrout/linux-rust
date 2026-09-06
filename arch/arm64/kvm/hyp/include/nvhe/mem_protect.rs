//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/nvhe/mem_protect.h
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
// Copyright (C) 2020 Google LLC
// Author: Quentin Perret <qperret@google.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_mmu {
    pub arch: kvm_arch,
    pub pgt: kvm_pgtable,
    pub mm_ops: kvm_pgtable_mm_ops,
    pub lock: hyp_spinlock_t,
}

// This corresponds to page-table locking order
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pkvm_component_id {
    PKVM_ID_HOST,
    PKVM_ID_HYP,
    PKVM_ID_GUEST,
}

extern "C" {
    pub fn __pkvm_prot_finalize() -> c_int;
}
extern "C" {
    pub fn __pkvm_host_share_hyp(pfn: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_guest_share_host(vcpu: *mut pkvm_hyp_vcpu, gfn: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_guest_unshare_host(vcpu: *mut pkvm_hyp_vcpu, gfn: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_unshare_hyp(pfn: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_donate_hyp(pfn: u64, nr_pages: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_hyp_donate_host(pfn: u64, nr_pages: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_share_ffa(pfn: u64, nr_pages: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_unshare_ffa(pfn: u64, nr_pages: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_donate_guest(pfn: u64, gfn: u64, vcpu: *mut pkvm_hyp_vcpu) -> c_int;
}
extern "C" {
    pub fn __pkvm_vcpu_in_poison_fault(hyp_vcpu: *mut pkvm_hyp_vcpu) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_force_reclaim_page_guest(phys: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_reclaim_page_guest(gfn: u64, vm: *mut pkvm_hyp_vm) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_unshare_guest(gfn: u64, nr_pages: u64, hyp_vm: *mut pkvm_hyp_vm) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_relax_perms_guest(gfn: u64, vcpu: *mut pkvm_hyp_vcpu, prot: kvm_pgtable_prot) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_wrprotect_guest(gfn: u64, nr_pages: u64, hyp_vm: *mut pkvm_hyp_vm) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_test_clear_young_guest(gfn: u64, nr_pages: u64, mkold: bool, vm: *mut pkvm_hyp_vm) -> c_int;
}
extern "C" {
    pub fn __pkvm_host_mkyoung_guest(gfn: u64, vcpu: *mut pkvm_hyp_vcpu) -> c_int;
}
extern "C" {
    pub fn addr_is_memory(phys: phys_addr_t) -> bool;
}
extern "C" {
    pub fn host_stage2_idmap_locked(addr: phys_addr_t, size: u64, prot: kvm_pgtable_prot) -> c_int;
}
extern "C" {
    pub fn host_stage2_set_owner_locked(addr: phys_addr_t, size: u64, owner_id: u8) -> c_int;
}
extern "C" {
    pub fn kvm_host_prepare_stage2(pgt_pool_base: *mut c_void) -> c_int;
}
extern "C" {
    pub fn kvm_guest_prepare_stage2(vm: *mut pkvm_hyp_vm, pgd: *mut c_void) -> c_int;
}
extern "C" {
    pub fn kvm_guest_destroy_stage2(vm: *mut pkvm_hyp_vm);
}
extern "C" {
    pub fn handle_host_mem_abort(host_ctxt: *mut kvm_cpu_context);
}
extern "C" {
    pub fn hyp_pin_shared_mem(from: *mut c_void, to: *mut c_void) -> c_int;
}
extern "C" {
    pub fn hyp_unpin_shared_mem(from: *mut c_void, to: *mut c_void);
}
extern "C" {
    pub fn reclaim_pgtable_pages(vm: *mut pkvm_hyp_vm, mc: *mut kvm_hyp_memcache);
}

extern "C" {
    pub fn pkvm_ownership_selftest(base: *mut c_void);
}
extern "C" {
    pub fn teardown_selftest_vm();
}

