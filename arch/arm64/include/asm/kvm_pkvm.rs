//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_pkvm.h
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
//
// Copyright (C) 2020 - Google LLC
// Author: Quentin Perret <qperret@google.com>
//

// Maximum number of VMs that can co-exist under pKVM.
pub const KVM_MAX_PVMS: c_int = 255;
pub const HYP_MEMBLOCK_REGIONS: c_int = 128;
extern "C" {
    pub fn pkvm_init_host_vm(kvm: *mut kvm, type: c_ulong) -> c_int;
}
extern "C" {
    pub fn pkvm_create_hyp_vm(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn pkvm_hyp_vm_is_created(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn pkvm_destroy_hyp_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn pkvm_create_hyp_vcpu(vcpu: *mut kvm_vcpu) -> c_int;
}
//
// Check whether the specific capability is allowed in pKVM.
//
// Certain features are allowed only for non-protected VMs in pKVM, which is why
// this takes the VM (kvm) as a parameter.
//
// Check whether the KVM VM IOCTL is allowed in pKVM.
//
// Certain features are allowed only for non-protected VMs in pKVM, which is why
// this takes the VM (kvm) as a parameter.
//
extern "C" {
    pub fn kvm_pkvm_ext_allowed(_arg: kvm, _arg: ext) -> return;
}
extern "C" {
    pub fn kvm_nvhe_sym(_arg: hyp_memblock_nr) -> c_uint;
}
// Provision the worst case scenario
// Cover all of memory with page-granularity
// Allow 1 GiB for private mappings
//
// Include an extra 16 pages to safely upper-bound the worst case of
// concatenated pgds.
//
// Allow 1 GiB for MMIO mappings

pub const KVM_FFA_MBOX_NR_PAGES: c_int = 1;
//
// The hypervisor FFA proxy needs enough memory to buffer a fragmented
// descriptor returned from EL3 in response to a RETRIEVE_REQ call.
//
extern "C" {
    pub fn sizeof(ffa_mem_region_addr_range: struct) -> *mut SG_MAX_SEGMENTS;
}
// Plus a page each for the hypervisor's RX and TX mailboxes.
extern "C" {
    pub fn SVE_SIG_REGS_SIZE(_arg: sve_vq_from_vl(kvm_host_sve_max_vl)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkvm_mapping {
    pub node: rb_node,
    pub gfn: u64,
    pub pfn: u64,
    pub nr_pages:48: u64,
    pub nc:1: u64,
}

extern "C" {
    pub fn pkvm_pgtable_stage2_destroy_pgd(pgt: *mut kvm_pgtable);
}
extern "C" {
    pub fn pkvm_pgtable_stage2_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn pkvm_pgtable_stage2_wrprotect(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn pkvm_pgtable_stage2_flush(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn pkvm_pgtable_stage2_test_clear_young(pgt: *mut kvm_pgtable, addr: u64, size: u64, mkold: bool) -> bool;
}
extern "C" {
    pub fn pkvm_pgtable_stage2_free_unlinked(mm_ops: *mut kvm_pgtable_mm_ops, pgtable: *mut c_void, level: i8);
}
