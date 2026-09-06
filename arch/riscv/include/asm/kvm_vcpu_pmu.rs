//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_vcpu_pmu.h
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
// Copyright (c) 2023 Rivos Inc
//
// Authors:
// Atish Patra <atishp@rivosinc.com>
//

pub const RISCV_KVM_MAX_FW_CTRS: c_int = 32;
pub const RISCV_KVM_MAX_HW_CTRS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fw_event {
// Current value of the event
    pub value: u64,
// Event monitoring status
    pub started: bool,
}

// Per virtual pmu counter data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmc {
    pub idx: u8,
    pub perf_event: *mut perf_event,
    pub counter_val: u64,
    pub cinfo: sbi_pmu_ctr_info,
// Event monitoring status
    pub started: bool,
// Monitoring event ID
    pub event_idx: c_ulong,
    pub vcpu: *mut kvm_vcpu,
}

// PMU data structure per vcpu
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu {
    pub pmc: [kvm_pmc; RISCV_KVM_MAX_COUNTERS],
    pub fw_event: [kvm_fw_event; RISCV_KVM_MAX_FW_CTRS],
// Number of the virtual firmware counters available
    pub num_fw_ctrs: c_int,
// Number of the virtual hardware counters available
    pub num_hw_ctrs: c_int,
// A flag to indicate that pmu initialization is done
    pub init_done: bool,
// Bit map of all the virtual counter used
    pub RISCV_KVM_MAX_COUNTERS): DECLARE_BITMAP(pmc_in_use,,
// Bit map of all the virtual counter overflown
    pub RISCV_KVM_MAX_COUNTERS): DECLARE_BITMAP(pmc_overflown,,
// The address of the counter snapshot area (guest physical address)
    pub snapshot_addr: gpa_t,
// The actual data of the snapshot
    pub sdata: *mut riscv_pmu_snapshot_data,
}

extern "C" {
    pub fn kvm_riscv_vcpu_pmu_incr_fw(vcpu: *mut kvm_vcpu, fid: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_pmu_num_ctrs(vcpu: *mut kvm_vcpu, retdata: *mut kvm_vcpu_sbi_return) -> c_int;
}
extern "C" {
    pub fn kvm_riscv_vcpu_pmu_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_pmu_deinit(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_riscv_vcpu_pmu_reset(vcpu: *mut kvm_vcpu);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu {
}

// val = 0;

