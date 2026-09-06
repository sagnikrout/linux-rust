//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/perf/riscv_pmu.h
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
// Copyright (C) 2018 SiFive
// Copyright (C) 2018 Andes Technology Corporation
// Copyright (C) 2021 Western Digital Corporation or its affiliates.
//

//
// The RISCV_MAX_COUNTERS parameter should be specified.
//
pub const RISCV_MAX_COUNTERS: c_int = 64;

pub const RISCV_PMU_STOP_FLAG_RESET: c_int = 1;
pub const RISCV_PMU_CONFIG1_GUEST_EVENTS: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_hw_events {
// currently enabled events
    pub n_events: c_int,
// Counter overflow interrupt
    pub irq: c_int,
// currently enabled events
    pub events: [*mut perf_event; RISCV_MAX_COUNTERS],
// currently enabled hardware counters
    pub RISCV_MAX_COUNTERS): DECLARE_BITMAP(used_hw_ctrs,,
// currently enabled firmware counters
    pub RISCV_MAX_COUNTERS): DECLARE_BITMAP(used_fw_ctrs,,
// The virtual address of the shared memory where counter snapshot will be taken
    pub snapshot_addr: *mut c_void,
// The physical address of the shared memory where counter snapshot will be taken
    pub snapshot_addr_phys: phys_addr_t,
// Boolean flag to indicate setup is already done
    pub snapshot_set_done: bool,
// A shadow copy of the counter values to avoid clobbering during multiple SBI calls
    pub snapshot_cval_shcopy: [u64; RISCV_MAX_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_pmu {
    pub pmu: pmu,
    pub name: *mut c_char,
    pub dev): *mut *mut irqreturn_t (handle_irq)(int irq_num, void,
    pub cmask: c_ulong,
    pub event): *mut *mut u64 (ctr_read)(struct perf_event,
    pub event): *mut *mut int (ctr_get_idx)(struct perf_event,
    pub idx): *mut *mut int (ctr_get_width)(int,
    pub event): *mut *mut void (ctr_clear_idx)(struct perf_event,
    pub init_val): *mut *mut *mut void (ctr_start)(struct perf_event event, u64,
    pub flag): *mut *mut *mut void (ctr_stop)(struct perf_event event, unsigned long,
    pub config): *mut *mut *mut int (event_map)(struct perf_event event, u64,
    pub event): *mut *mut void (event_init)(struct perf_event,
    pub mm): *mut *mut *mut void (event_mapped)(struct perf_event event, struct mm_struct,
    pub mm): *mut *mut *mut void (event_unmapped)(struct perf_event event, struct mm_struct,
    pub event): *mut *mut uint8_t (csr_index)(struct perf_event,
    pub hw_events: *mut cpu_hw_events __percpu,
    pub node: hlist_node,
    pub riscv_pm_nb: notifier_block,
}

extern "C" {
    pub fn riscv_pmu_start(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn riscv_pmu_stop(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn riscv_pmu_ctr_read_csr(csr: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn riscv_pmu_event_set_period(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn riscv_pmu_ctr_get_width_mask(event: *mut perf_event) -> u64;
}
extern "C" {
    pub fn riscv_pmu_event_update(event: *mut perf_event) -> u64;
}

extern "C" {
    pub fn riscv_pmu_legacy_skip_init();
}

extern "C" {
    pub fn riscv_pmu_get_hpm_info(hw_ctr_width: *mut u32, num_hw_ctr: *mut u32) -> c_int;
}
extern "C" {
    pub fn riscv_pmu_get_event_info(type: u32, config: u64, econfig: *mut u64) -> c_int;
}

