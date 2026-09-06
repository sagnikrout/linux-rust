//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kvm/e500.h
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
// Copyright (C) 2008-2011 Freescale Semiconductor, Inc. All rights reserved.
//
// Author: Yu Liu <yu.liu@freescale.com>
// Scott Wood <scottwood@freescale.com>
// Ashish Kalra <ashish.kalra@freescale.com>
// Varun Sethi <varun.sethi@freescale.com>
//
// Description:
// This file is based on arch/powerpc/kvm/44x_tlb.h and
// arch/powerpc/include/asm/kvm_44x.h by Hollis Blanchard <hollisb@us.ibm.com>,
// Copyright IBM Corp. 2007-2008
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcpu_ftr {
    VCPU_FTR_MMU_V2
}

pub const E500_PID_NUM: c_int = 3;
pub const E500_TLB_NUM: c_int = 2;
// entry is mapped somewhere in host TLB

// TLB1 entry is mapped by host TLB1, tracked by bitmaps

// TLB1 entry is mapped by host TLB0

// entry is writable on the host

// bits [6-5] MAS2_X1 and MAS2_X0 and [4-0] bits for WIMGE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlbe_priv {
    pub /: *mut *mut kvm_pfn_t pfn; / valid only for TLB0, except briefly,
    pub /: *mut *mut *mut unsigned int flags; / E500_TLB_,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_e500_tlb_params {
    pub sets: int entries, ways,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_vcpu_e500 {
    pub vcpu: kvm_vcpu,
// Unmodified copy of the guest's TLB -- shared with host userspace.
    pub gtlb_arch: *mut kvm_book3e_206_tlb_entry,
// Starting entry number in gtlb_arch[]
    pub gtlb_offset: [c_int; E500_TLB_NUM],
// KVM internal information associated with each guest TLB entry
    pub gtlb_priv: [*mut tlbe_priv; E500_TLB_NUM],
    pub gtlb_params: [kvmppc_e500_tlb_params; E500_TLB_NUM],
    pub gtlb_nv: [c_uint; E500_TLB_NUM],
    pub host_tlb1_nv: c_uint,
    pub svr: u32,
    pub l1csr0: u32,
    pub l1csr1: u32,
    pub hid0: u32,
    pub hid1: u32,
    pub mcar: u64,
    pub shared_tlb_pages: *mut page,
    pub num_shared_tlb_pages: c_int,
    pub g2h_tlb1_map: *mut u64,
    pub h2g_tlb1_rmap: *mut c_uint,
// Minimum and maximum address mapped my TLB1
    pub tlb1_min_eaddr: c_ulong,
    pub tlb1_max_eaddr: c_ulong,
    pub pid: [u32; E500_PID_NUM],
// vcpu id table
    pub idt: *mut vcpu_id_table,

}

extern "C" {
    pub fn container_of(_arg: vcpu, kvmppc_vcpu_e500: struct, _arg: vcpu) -> return;
}
// This geometry is the legacy default -- can be overridden by userspace
pub const KVM_E500_TLB0_WAY_SIZE: c_int = 128;
pub const KVM_E500_TLB0_WAY_NUM: c_int = 2;

pub const KVM_E500_TLB1_SIZE: c_int = 16;

extern "C" {
    pub fn kvmppc_e500_emul_tlbwe(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_e500_emul_tlbre(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_e500_emul_tlbivax(vcpu: *mut kvm_vcpu, ea: gva_t) -> c_int;
}
extern "C" {
    pub fn kvmppc_e500_emul_tlbilx(vcpu: *mut kvm_vcpu, type: c_int, ea: gva_t) -> c_int;
}
extern "C" {
    pub fn kvmppc_e500_emul_tlbsx(vcpu: *mut kvm_vcpu, ea: gva_t) -> c_int;
}
extern "C" {
    pub fn kvmppc_e500_tlb_init(vcpu_e500: *mut kvmppc_vcpu_e500) -> c_int;
}
extern "C" {
    pub fn kvmppc_e500_tlb_uninit(vcpu_e500: *mut kvmppc_vcpu_e500);
}
extern "C" {
    pub fn kvmppc_get_sregs_e500_tlb(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs);
}
extern "C" {
    pub fn kvmppc_set_sregs_e500_tlb(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs) -> c_int;
}

// TLB helper functions
//
// Manual says that tlbsel has 2 bits wide.
// Since we only have two TLBs, only lower bit is used.
//

// Does it match current guest AS?
// XXX what about IS != DS?

// Mapping is not for RAM.
extern "C" {
    pub fn kvmppc_e500_tlbil_all(vcpu_e500: *mut kvmppc_vcpu_e500);
}

//
// These functions should be called with preemption disabled
// and the returned value is valid only in that context
//
extern "C" {
    pub fn get_thread_specific_lpid(_arg: vcpu->kvm->arch.lpid) -> return;
}

// Force TS=1 for all guest mappings.

