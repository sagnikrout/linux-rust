//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/hyperv.h
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
// KVM Microsoft Hyper-V emulation
//
// derived from arch/x86/kvm/x86.c
//
// Copyright (C) 2006 Qumranet, Inc.
// Copyright (C) 2008 Qumranet, Inc.
// Copyright IBM Corporation, 2008
// Copyright 2010 Red Hat, Inc. and/or its affiliates.
// Copyright (C) 2015 Andrey Smetanin <asmetanin@virtuozzo.com>
//
// Authors:
// Avi Kivity   <avi@qumranet.com>
// Yaniv Kamay  <yaniv@qumranet.com>
// Amit Shah    <amit.shah@qumranet.com>
// Ben-Ami Yassour <benami@il.ibm.com>
// Andrey Smetanin <asmetanin@virtuozzo.com>
//

// Hyper-V SynIC timer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_hv_stimer {
    pub timer: hrtimer,
    pub index: c_int,
    pub config: hv_stimer_config,
    pub count: u64,
    pub exp_time: u64,
    pub msg: hv_message,
    pub msg_pending: bool,
}

// Hyper-V synthetic interrupt controller (SynIC)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_hv_synic {
    pub version: u64,
    pub control: u64,
    pub msg_page: u64,
    pub evt_page: u64,
    pub sint: [core::sync::atomic::AtomicI64; HV_SYNIC_SINT_COUNT],
    pub sint_to_gsi: [core::sync::atomic::AtomicI32; HV_SYNIC_SINT_COUNT],
    pub 256): DECLARE_BITMAP(auto_eoi_bitmap,,
    pub 256): DECLARE_BITMAP(vec_bitmap,,
    pub active: bool,
    pub dont_zero_synic_pages: bool,
}

// The maximum number of entries on the TLB flush fifo.

//
// Note: the following 'magic' entry is made up by KVM to avoid putting
// anything besides GVA on the TLB flush fifo. It is theoretically possible
// to observe a request to flush 4095 PFNs starting from 0xfffffffffffff000
// which will look identical. KVM's action to 'flush everything' instead of
// flushing these particular addresses is, however, fully legitimate as
// flushing more than requested is always OK.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_tlb_flush_fifos {
    HV_L1_TLB_FLUSH_FIFO,
    HV_L2_TLB_FLUSH_FIFO,
    HV_NR_TLB_FLUSH_FIFOS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_hv_tlb_flush_fifo {
    pub write_lock: spinlock_t,
    pub KVM_HV_TLB_FLUSH_FIFO_SIZE): DECLARE_KFIFO(entries, u64,,
}

// Hyper-V per vcpu emulation context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_hv {
    pub vcpu: *mut kvm_vcpu,
    pub vp_index: u32,
    pub hv_vapic: u64,
    pub runtime_offset: i64,
    pub synic: kvm_vcpu_hv_synic,
    pub exit: kvm_hyperv_exit,
    pub stimer: [kvm_vcpu_hv_stimer; HV_SYNIC_STIMER_COUNT],
    pub HV_SYNIC_STIMER_COUNT): DECLARE_BITMAP(stimer_pending_bitmap,,
    pub enforce_cpuid: bool,
    pub /: *mut *mut u32 features_eax; / HYPERV_CPUID_FEATURES.EAX,
    pub /: *mut *mut u32 features_ebx; / HYPERV_CPUID_FEATURES.EBX,
    pub /: *mut *mut u32 features_edx; / HYPERV_CPUID_FEATURES.EDX,
    pub /: *mut *mut u32 enlightenments_eax; / HYPERV_CPUID_ENLIGHTMENT_INFO.EAX,
    pub /: *mut *mut u32 enlightenments_ebx; / HYPERV_CPUID_ENLIGHTMENT_INFO.EBX,
    pub /: *mut *mut u32 syndbg_cap_eax; / HYPERV_CPUID_SYNDBG_PLATFORM_CAPABILITIES.EAX,
    pub /: *mut *mut u32 nested_eax; / HYPERV_CPUID_NESTED_FEATURES.EAX,
    pub /: *mut *mut u32 nested_ebx; / HYPERV_CPUID_NESTED_FEATURES.EBX,
    pub cpuid_cache: },
    pub tlb_flush_fifo: [kvm_vcpu_hv_tlb_flush_fifo; HV_NR_TLB_FLUSH_FIFOS],
//
// Preallocated buffers for handling hypercalls that pass sparse vCPU
// sets (for high vCPU counts, they're too large to comfortably fit on
// the stack).
//
    pub sparse_banks: [u64; HV_MAX_SPARSE_VCPU_BANKS],
    pub KVM_MAX_VCPUS): DECLARE_BITMAP(vcpu_mask,,
    pub vp_assist_page: hv_vp_assist_page,
    pub pa_page_gpa: u64,
    pub vm_id: u64,
    pub vp_id: u32,
    pub nested: },
}

// "Hv#1" signature
pub const HYPERV_CPUID_SIGNATURE_EAX: c_uint = 0x31237648;
//
// The #defines related to the synthetic debugger are required by KDNet, but
// they are not documented in the Hyper-V TLFS because the synthetic debugger
// functionality has been deprecated and is subject to removal in future
// versions of Windows.
//
pub const HYPERV_CPUID_SYNDBG_VENDOR_AND_MAX_FUNCTIONS: c_uint = 0x40000080;
pub const HYPERV_CPUID_SYNDBG_INTERFACE: c_uint = 0x40000081;
pub const HYPERV_CPUID_SYNDBG_PLATFORM_CAPABILITIES: c_uint = 0x40000082;
//
// Hyper-V synthetic debugger platform capabilities
// These are HYPERV_CPUID_SYNDBG_PLATFORM_CAPABILITIES.EAX bits.
//

// Hyper-V Synthetic debug options MSR
pub const HV_X64_MSR_SYNDBG_CONTROL: c_uint = 0x400000F1;
pub const HV_X64_MSR_SYNDBG_STATUS: c_uint = 0x400000F2;
pub const HV_X64_MSR_SYNDBG_SEND_BUFFER: c_uint = 0x400000F3;
pub const HV_X64_MSR_SYNDBG_RECV_BUFFER: c_uint = 0x400000F4;
pub const HV_X64_MSR_SYNDBG_PENDING_BUFFER: c_uint = 0x400000F5;
pub const HV_X64_MSR_SYNDBG_OPTIONS: c_uint = 0x400000FF;
// Hyper-V HV_X64_MSR_SYNDBG_OPTIONS bits

//
// Ensure the HyperV structure is fully initialized when accessing it
// without holding vcpu->mutex (or some other guarantee that KVM can't
// concurrently instantiate the structure).
//
// Pairs with the smp_store_release() in kvm_hv_vcpu_init().
//
extern "C" {
    pub fn smp_load_acquire(_arg: &vcpu->arch.hyperv) -> return;
}
extern "C" {
    pub fn kvm_hv_set_msr_common(vcpu: *mut kvm_vcpu, msr: u32, data: u64, host: bool) -> c_int;
}
extern "C" {
    pub fn kvm_hv_get_msr_common(vcpu: *mut kvm_vcpu, msr: u32, pdata: *mut u64, host: bool) -> c_int;
}
extern "C" {
    pub fn kvm_hv_hypercall(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_hv_irq_routing_update(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_hv_synic_send_eoi(vcpu: *mut kvm_vcpu, vector: c_int);
}
extern "C" {
    pub fn kvm_hv_activate_synic(vcpu: *mut kvm_vcpu, dont_zero_synic_pages: bool) -> c_int;
}
extern "C" {
    pub fn to_hv_vcpu(test_bit(vector: vcpu) &&, _arg: to_hv_synic(vcpu)->vec_bitmap) -> return;
}
extern "C" {
    pub fn kvm_hv_vcpu_uninit(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_hv_assist_page_enabled(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_hv_get_assist_page(vcpu: *mut kvm_vcpu) -> c_int;
}
//
// With HV_ACCESS_TSC_INVARIANT feature, invariant TSC (CPUID.80000007H:EDX[8])
// is only observed after HV_X64_MSR_TSC_INVARIANT_CONTROL was written to.
//
// If Hyper-V's invariant TSC control is not exposed to the guest,
// the invariant TSC CPUID flag is not suppressed, Windows guests were
// observed to be able to handle it correctly. Going forward, VMMs are
// encouraged to enable Hyper-V's invariant TSC control when invariant
// TSC CPUID flag is set to make KVM's behavior match genuine Hyper-V.
//
// If Hyper-V's invariant TSC control is exposed to the guest, KVM is
// responsible for suppressing the invariant TSC CPUID flag if the
// Hyper-V control is not enabled.
//
extern "C" {
    pub fn kvm_hv_process_stimers(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_hv_request_tsc_page_update(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_hv_xsaves_xsavec_maybe_warn(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_hv_init_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_hv_destroy_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_hv_vcpu_init(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_hv_set_cpuid(vcpu: *mut kvm_vcpu, hyperv_enabled: bool);
}
extern "C" {
    pub fn kvm_hv_set_enforce_cpuid(vcpu: *mut kvm_vcpu, enforce: bool) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_hv_eventfd(kvm: *mut kvm, args: *mut kvm_hyperv_eventfd) -> c_int;
}
extern "C" {
    pub fn kvm_hv_get_assist_page(_arg: vcpu) -> return;
}
//
// KVM_REQ_HV_TLB_FLUSH flushes entries from either L1's VP_ID or
// L2's VP_ID upon request from the guest. Make sure we check for
// pending entries in the right FIFO upon L1/L2 transition as these
// requests are put by other vCPUs asynchronously.
//
extern "C" {
    pub fn kvm_hv_vcpu_flush_tlb(vcpu: *mut kvm_vcpu) -> c_int;
}

