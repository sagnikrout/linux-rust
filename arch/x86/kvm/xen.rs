//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/xen.h
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
// Copyright © 2019 Oracle and/or its affiliates. All rights reserved.
// Copyright © 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// KVM Xen emulation
//

extern "C" {
    pub fn __kvm_xen_has_interrupt(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_xen_inject_pending_events(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_xen_inject_vcpu_vector(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_xen_vcpu_set_attr(vcpu: *mut kvm_vcpu, data: *mut kvm_xen_vcpu_attr) -> c_int;
}
extern "C" {
    pub fn kvm_xen_vcpu_get_attr(vcpu: *mut kvm_vcpu, data: *mut kvm_xen_vcpu_attr) -> c_int;
}
extern "C" {
    pub fn kvm_xen_hvm_set_attr(kvm: *mut kvm, data: *mut kvm_xen_hvm_attr) -> c_int;
}
extern "C" {
    pub fn kvm_xen_hvm_get_attr(kvm: *mut kvm, data: *mut kvm_xen_hvm_attr) -> c_int;
}
extern "C" {
    pub fn kvm_xen_hvm_evtchn_send(kvm: *mut kvm, evt: *mut kvm_irq_routing_xen_evtchn) -> c_int;
}
extern "C" {
    pub fn kvm_xen_write_hypercall_page(vcpu: *mut kvm_vcpu, data: u64) -> c_int;
}
extern "C" {
    pub fn kvm_xen_hvm_config(kvm: *mut kvm, xhc: *mut kvm_xen_hvm_config) -> c_int;
}
extern "C" {
    pub fn kvm_xen_init_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_xen_destroy_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_xen_init_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_xen_destroy_vcpu(vcpu: *mut kvm_vcpu);
}
//
// The local APIC is being enabled. If the per-vCPU upcall vector is
// set and the vCPU's evtchn_upcall_pending flag is set, inject the
// interrupt.
//
extern "C" {
    pub fn __kvm_xen_has_interrupt(_arg: vcpu) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &vcpu->arch.xen.timer_pending) -> return;
}
extern "C" {
    pub fn kvm_xen_inject_timer_irqs(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvm_xen_hypercall(vcpu: *mut kvm_vcpu) -> c_int;
}

extern "C" {
    pub fn kvm_xen_update_runstate(vcpu: *mut kvm_vcpu, state: c_int);
}
//
// If the vCPU wasn't preempted but took a normal exit for
// some reason (hypercalls, I/O, etc.), that is accounted as
// still RUNSTATE_running, as the VMM is still operating on
// behalf of the vCPU. Only if the VMM does actually block
// does it need to enter RUNSTATE_blocked.
//
// 32-bit compatibility definitions, also used natively in 32-bit build
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_arch_vcpu_info {
    pub cr2: c_uint,
    pub pad: [c_uint; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_vcpu_info {
    pub evtchn_upcall_pending: u8,
    pub evtchn_upcall_mask: u8,
    pub pad: u16,
    pub evtchn_pending_sel: u32,
    pub arch: compat_arch_vcpu_info,
    pub time: pvclock_vcpu_time_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_arch_shared_info {
    pub max_pfn: c_uint,
    pub pfn_to_mfn_frame_list_list: c_uint,
    pub nmi_reason: c_uint,
    pub p2m_cr3: c_uint,
    pub p2m_vaddr: c_uint,
    pub p2m_generation: c_uint,
    pub wc_sec_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_shared_info {
    pub vcpu_info: [compat_vcpu_info; MAX_VIRT_CPUS],
    pub evtchn_pending: [u32; 32],
    pub evtchn_mask: [u32; 32],
    pub wc: pvclock_wall_clock,
    pub arch: compat_arch_shared_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_vcpu_runstate_info {
    pub state: c_int,
    pub state_entry_time: u64,
    pub time: [u64; 4],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_sched_poll {
// This is actually a guest virtual address which points to ports.
    pub ports: u32,
    pub nr_ports: c_uint,
    pub timeout: u64,
}
