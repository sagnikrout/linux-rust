//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/nvhe/pkvm.h
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
// Copyright (C) 2021 Google LLC
// Author: Fuad Tabba <tabba@google.com>
//

//
// Holds the relevant data for maintaining the vcpu state completely at hyp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkvm_hyp_vcpu {
    pub vcpu: kvm_vcpu,
// Backpointer to the host's (untrusted) vCPU instance.
    pub host_vcpu: *mut kvm_vcpu,
//
// If this hyp vCPU is loaded, then this is a backpointer to the
// per-cpu pointer tracking us. Otherwise, NULL if not loaded.
//
    pub loaded_hyp_vcpu: *mut pkvm_hyp_vcpu,
}

//
// Holds the relevant data for running a vm in protected mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkvm_hyp_vm {
    pub kvm: kvm,
// Backpointer to the host's (untrusted) KVM instance.
    pub host_kvm: *mut kvm,
// The guest's stage-2 page-table managed by the hypervisor.
    pub pgt: kvm_pgtable,
    pub mm_ops: kvm_pgtable_mm_ops,
    pub pool: hyp_pool,
    pub lock: hyp_spinlock_t,
// Array of the hyp vCPU structures for this VM.
    pub vcpus: [*mut pkvm_hyp_vcpu; ],
}

extern "C" {
    pub fn container_of(_arg: hyp_vcpu->vcpu.kvm, pkvm_hyp_vm: struct, _arg: kvm) -> return;
}
extern "C" {
    pub fn vcpu_is_protected(_arg: &hyp_vcpu->vcpu) -> return;
}
extern "C" {
    pub fn kvm_vm_is_protected(_arg: &hyp_vm->kvm) -> return;
}
extern "C" {
    pub fn pkvm_hyp_vm_table_init(tbl: *mut c_void);
}
extern "C" {
    pub fn __pkvm_reserve_vm() -> c_int;
}
extern "C" {
    pub fn __pkvm_unreserve_vm(handle: pkvm_handle_t);
}
extern "C" {
    pub fn __pkvm_reclaim_dying_guest_page(handle: pkvm_handle_t, gfn: u64) -> c_int;
}
extern "C" {
    pub fn __pkvm_start_teardown_vm(handle: pkvm_handle_t) -> c_int;
}
extern "C" {
    pub fn __pkvm_finalize_teardown_vm(handle: pkvm_handle_t) -> c_int;
}
extern "C" {
    pub fn pkvm_put_hyp_vcpu(hyp_vcpu: *mut pkvm_hyp_vcpu);
}
extern "C" {
    pub fn put_pkvm_hyp_vm(hyp_vm: *mut pkvm_hyp_vm);
}
extern "C" {
    pub fn kvm_handle_pvm_hvc64(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool;
}
extern "C" {
    pub fn kvm_handle_pvm_sysreg(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool;
}
extern "C" {
    pub fn kvm_handle_pvm_restricted(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool;
}
extern "C" {
    pub fn kvm_init_pvm_id_regs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_check_pvm_sysreg_table() -> c_int;
}
