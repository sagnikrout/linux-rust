//! Automatically rewritten from C Header to Rust Module
//! Source: include/kvm/arm_pmu.h
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
// Copyright (C) 2015 Linaro Ltd.
// Author: Shannon Zhao <shannon.zhao@linaro.org>
//

pub const KVM_ARMV8_PMU_MAX_COUNTERS: c_int = 32;
// PPI #23 - architecturally specified for GICv5
pub const KVM_ARMV8_PMU_GICV5_IRQ: c_uint = 0x20000017;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmc {
    pub /: *mut *mut u8 idx; / index into the pmu->pmc array,
    pub perf_event: *mut perf_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu_events {
    pub events_host: u64,
    pub events_guest: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu {
    pub overflow_work: irq_work,
    pub events: kvm_pmu_events,
    pub pmc: [kvm_pmc; KVM_ARMV8_PMU_MAX_COUNTERS],
    pub irq_num: c_int,
    pub created: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_pmu_entry {
    pub entry: list_head,
    pub arm_pmu: *mut arm_pmu,
}

extern "C" {
    pub fn kvm_supports_guest_pmuv3() -> bool;
}

extern "C" {
    pub fn kvm_pmu_get_counter_value(vcpu: *mut kvm_vcpu, select_idx: u64) -> u64;
}
extern "C" {
    pub fn kvm_pmu_set_counter_value(vcpu: *mut kvm_vcpu, select_idx: u64, val: u64);
}
extern "C" {
    pub fn kvm_pmu_set_counter_value_user(vcpu: *mut kvm_vcpu, select_idx: u64, val: u64);
}
extern "C" {
    pub fn kvm_pmu_implemented_counter_mask(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn kvm_pmu_accessible_counter_mask(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn kvm_pmu_get_pmceid(vcpu: *mut kvm_vcpu, pmceid1: bool) -> u64;
}
extern "C" {
    pub fn kvm_pmu_vcpu_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_vcpu_destroy(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_reprogram_counter_mask(vcpu: *mut kvm_vcpu, val: u64);
}
extern "C" {
    pub fn kvm_pmu_flush_hwstate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_sync_hwstate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_pmu_should_notify_user(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_pmu_update_run(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_pmu_software_increment(vcpu: *mut kvm_vcpu, val: u64);
}
extern "C" {
    pub fn kvm_pmu_handle_pmcr(vcpu: *mut kvm_vcpu, val: u64);
}
extern "C" {
    pub fn kvm_vcpu_reload_pmu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arm_pmu_v3_enable(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_vcpu_pmu_restore_guest(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_pmu_restore_host(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vcpu_pmu_resync_el0();
}

//
// Updates the vcpu's view of the pmu events for this cpu.
// Must be called before every vcpu run after disabling interrupts, to ensure
// that an interrupt cannot fire and update the structure.
//

extern "C" {
    pub fn kvm_arm_pmu_get_pmuver_limit() -> u8;
}
extern "C" {
    pub fn kvm_pmu_evtyper_mask(kvm: *mut kvm) -> u64;
}
extern "C" {
    pub fn kvm_arm_set_default_pmu(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_arm_pmu_get_max_counters(kvm: *mut kvm) -> u8;
}
extern "C" {
    pub fn kvm_vcpu_read_pmcr(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn kvm_pmu_counter_is_hyp(vcpu: *mut kvm_vcpu, idx: c_uint) -> bool;
}
extern "C" {
    pub fn kvm_pmu_nested_transition(vcpu: *mut kvm_vcpu);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu {
}

